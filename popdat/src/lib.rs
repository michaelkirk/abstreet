//! popdat ("population data") generates `Scenarios` given a map and some external census data.
//! Some of this functionality should maybe be reorganized or incorporated into the importer crate,
//! but for now, it's convenient to organize it here.
//!
//! All of the types and methods here are tied to a single `Map`. Even if a city is chopped up into
//! multiple pieces, for now, let's assume we're just dealing with one map at a time. That lets us
//! use the map's coordinate system, building IDs, etc.
//!
//! These types form a pipeline:
//!
//! 1) For a given map, find some census data that describes how many people live in different
//!    areas of the city. (CensusArea)
//! 2) Take the CensusAreas and turn them into individual CensusPersons, by randomly choosing a
//!    specific building on the map as their home, and assigning specific attributes based on the
//!    census data's distribution.
//! 3) For each CensusPerson, classify them into a PersonType, then generate a Schedule of
//!    different Activities throughout the day.
//! 4) Pick specific buildings to visit to satisfy the Schedule.

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use rand_xorshift::XorShiftRng;

use abstutil::Timer;
use geom::{Distance, Time};
use map_model::{BuildingID, Map};
use sim::Scenario;

mod activities;
mod distribute_people;
mod import_census;
mod make_person;

/// Represents aggregate demographic data for some part of a city. These could be census tracts or
/// blocks, depending what data we find. All of the areas should roughly partition the map -- we
/// probably don't need to guarantee we cover every single building, but we definitely shouldn't
/// have two overlapping areas.
#[derive(Debug, PartialEq)]
pub struct CensusArea {
    pub polygon: geo::Polygon<f64>,
    pub population: usize,
    // TODO Not sure what goes here, whatever census data actually has that could be useful
}

/// Demographic information for a single person
pub struct CensusPerson {
    pub home: BuildingID,
    pub age: usize,
    pub employed: bool,
    pub owns_car: bool,
}

/// It might be useful to classify a CensusPerson into different categories to figure out their
/// Schedule.
pub enum PersonType {
    Student,
    Worker,
}

/// A single person's daily schedule. It's assumed that someone always starts at home. And for most
/// people, the last entry should probably be Activity::Home.
pub struct Schedule {
    pub activities: Vec<(Time, Activity)>,
}

/// Different things people might do in the day. Maybe it's more clear to call this a
/// DestinationType or similar.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Activity {
    Breakfast,
    Lunch,
    Dinner,
    School,
    Entertainment,
    Errands,
    Financial,
    Healthcare,
    Home,
    Work,
}

/// Any arbitrarily chosen parameters needed should be put here, so they can be controlled from the
/// UI or tuned for different cities.
pub struct Config {
    pub walk_for_distances_shorter_than: Distance,
    pub walk_or_bike_for_distances_shorter_than: Distance,
}

impl Config {
    pub fn default() -> Config {
        Config {
            walk_for_distances_shorter_than: Distance::miles(0.5),
            walk_or_bike_for_distances_shorter_than: Distance::miles(3.0),
        }
    }
}

/// Wires together all the pieces, so you can just hand this any map, and it'll automatically find
/// appropriate census data, and use it to produce a Scenario.
pub async fn scenario_builder<'a, 'b>(
    scenario_name: &str,
    config: Config,
    map_area: geom::Polygon,
    map_bounds: geom::GPSBounds,
    rng: &mut XorShiftRng,
) -> anyhow::Result<Box<dyn FnOnce(&Map) -> Scenario>> {
    let mut timer = Timer::new("generate census scenario");
    timer.start("building population areas for map");
    let areas = CensusArea::fetch_all_for_map(&map_area, &map_bounds, &mut timer).await?;
    timer.stop("building population areas for map");
    let scenario_name = scenario_name.to_string();
    let rng = sim::fork_rng(rng);
    let builder: Box<dyn FnOnce(&Map) -> Scenario> =
        Box::new(move |map| generate_scenario(scenario_name, areas, config, map, rng, timer));
    Ok(builder)
}

pub fn generate_scenario(
    scenario_name: String,
    areas: Vec<CensusArea>,
    config: Config,
    map: &Map,
    mut rng: XorShiftRng,
    mut timer: Timer,
) -> Scenario {
    // find_data_for_map may return an error. If so, just plumb it back to the caller using the ?
    // operator
    timer.start("assigning people to houses");
    let people = distribute_people::assign_people_to_houses(areas, map, &mut rng, &config);
    timer.stop("assigning people to houses");

    let mut scenario = Scenario::empty(map, &scenario_name);
    timer.start("building people");
    scenario.people.extend(make_person::make_people(
        people, map, &mut timer, &mut rng, &config,
    ));
    timer.stop("building people");

    timer.start("removing weird schedules");
    scenario = scenario.remove_weird_schedules();
    timer.stop("removing weird schedules");

    scenario
}
