use std::collections::BTreeMap;

use geom::Pt2D;
use map_gui::tools::{grey_out_map, PopupMsg};
use map_gui::ID;
use map_model::{AreaID, BuildingID, BusRouteID, IntersectionID, LaneID, ParkingLotID, RoadID};
use sim::{PedestrianID, PersonID, TripID};
use widgetry::{
    EventCtx, GfxCtx, Key, Outcome, Panel, State, Text, TextExt, TextSpan, Warper, Widget,
};

use crate::app::{App, PerMap, Transition};
use crate::info::{OpenTrip, Tab};
use crate::sandbox::SandboxMode;

const WARP_TO_CAM_ZOOM: f64 = 10.0;

pub struct Warping {
    warper: Warper,
    id: Option<ID>,
}

impl Warping {
    pub fn new(
        ctx: &EventCtx,
        pt: Pt2D,
        target_cam_zoom: Option<f64>,
        id: Option<ID>,
        primary: &mut PerMap,
    ) -> Box<dyn State<App>> {
        primary.last_warped_from = Some((ctx.canvas.center_to_map_pt(), ctx.canvas.cam_zoom));
        Box::new(Warping {
            warper: Warper::new(ctx, pt, target_cam_zoom),
            id,
        })
    }
}

impl State<App> for Warping {
    fn event(&mut self, ctx: &mut EventCtx, _: &mut App) -> Transition {
        if self.warper.event(ctx) {
            Transition::Keep
        } else {
            if let Some(id) = self.id.clone() {
                Transition::Multi(vec![
                    Transition::Pop,
                    Transition::ModifyState(Box::new(move |state, ctx, app| {
                        // Other states pretty much don't use info panels.
                        if let Some(ref mut s) = state.downcast_mut::<SandboxMode>() {
                            let mut actions = s.contextual_actions();
                            s.controls.common.as_mut().unwrap().launch_info_panel(
                                ctx,
                                app,
                                Tab::from_id(app, id),
                                &mut actions,
                            );
                        }
                    })),
                ])
            } else {
                Transition::Pop
            }
        }
    }

    fn draw(&self, _: &mut GfxCtx, _: &App) {}
}

pub struct DebugWarp {
    panel: Panel,
}

impl DebugWarp {
    pub fn new(ctx: &mut EventCtx) -> Box<dyn State<App>> {
        let c = ctx.style().text_hotkey_color;
        Box::new(DebugWarp {
            panel: Panel::new(Widget::col(vec![
                Widget::row(vec![
                    "Warp to an object by ID"
                        .span()
                        .small_heading()
                        .into_widget(ctx),
                    ctx.style().btn_close_widget(ctx),
                ]),
                "Example: r42 is Road #42".text_widget(ctx),
                // T
                // his
                //
                // i
                // s
                //
                // d
                // isorienting...
                Text::from_all(vec![
                    "r".span().fg(c),
                    "oad, ".span(),
                    "l".span().fg(c),
                    "ane, ".span(),
                    "i".span().fg(c),
                    "ntersection, ".span(),
                    "b".span().fg(c),
                    "uilding, ".span(),
                    "p".span().fg(c),
                    "edestrian, ".span(),
                    "c".span().fg(c),
                    "ar, ".span(),
                    "t".span().fg(c),
                    "rip, ".span(),
                    "P".span().fg(c),
                    "erson, ".span(),
                    "R".span().fg(c),
                    "oute, parking ".span(),
                    "L".span().fg(c),
                    "ot".span(),
                ])
                .into_widget(ctx),
                Text::from_all(vec![
                    "Or ".span(),
                    "j".span().fg(c),
                    "ump to the previous position".span(),
                ])
                .into_widget(ctx),
                Widget::text_entry(ctx, String::new(), true).named("input"),
                ctx.style()
                    .btn_outline
                    .text("Go!")
                    .hotkey(Key::Enter)
                    .build_def(ctx),
            ]))
            .build(ctx),
        })
    }
}

impl State<App> for DebugWarp {
    fn event(&mut self, ctx: &mut EventCtx, app: &mut App) -> Transition {
        match self.panel.event(ctx) {
            Outcome::Clicked(x) => match x.as_ref() {
                "close" => {
                    return Transition::Pop;
                }
                "Go!" => {
                    let input = self.panel.text_box("input");
                    warp_to_id(ctx, app, &input)
                }
                _ => unreachable!(),
            },
            _ => Transition::Keep,
        }
    }

    fn draw(&self, g: &mut GfxCtx, app: &App) {
        grey_out_map(g, app);
        self.panel.draw(g);
    }
}

pub fn warp_to_id(ctx: &mut EventCtx, app: &mut App, input: &str) -> Transition {
    if let Some(t) = inner_warp_to_id(ctx, app, input) {
        t
    } else {
        Transition::Replace(PopupMsg::new(
            ctx,
            "Bad warp ID",
            vec![format!("{} isn't a valid ID", input)],
        ))
    }
}

fn inner_warp_to_id(ctx: &mut EventCtx, app: &mut App, line: &str) -> Option<Transition> {
    if line.is_empty() {
        return None;
    }
    if line == "j" {
        if let Some((pt, zoom)) = app.primary.last_warped_from {
            return Some(Transition::Replace(Warping::new(
                ctx,
                pt,
                Some(zoom),
                None,
                &mut app.primary,
            )));
        }
        return None;
    }

    let id = match usize::from_str_radix(&line[1..line.len()], 10) {
        Ok(idx) => match line.chars().next().unwrap() {
            'r' => {
                let r = app.primary.map.maybe_get_r(RoadID(idx))?;
                ID::Lane(r.lanes_ltr()[0].0)
            }
            'R' => {
                let r = BusRouteID(idx);
                app.primary.map.maybe_get_br(r)?;
                return Some(Transition::Multi(vec![
                    Transition::Pop,
                    Transition::ModifyState(Box::new(move |state, ctx, app| {
                        // Other states pretty much don't use info panels.
                        if let Some(ref mut s) = state.downcast_mut::<SandboxMode>() {
                            let mut actions = s.contextual_actions();
                            s.controls.common.as_mut().unwrap().launch_info_panel(
                                ctx,
                                app,
                                Tab::BusRoute(r),
                                &mut actions,
                            );
                        }
                    })),
                ]));
            }
            'l' => ID::Lane(LaneID(idx)),
            'L' => ID::ParkingLot(ParkingLotID(idx)),
            'i' => ID::Intersection(IntersectionID(idx)),
            'b' => ID::Building(BuildingID(idx)),
            'a' => ID::Area(AreaID(idx)),
            'p' => ID::Pedestrian(PedestrianID(idx)),
            'P' => {
                let id = PersonID(idx);
                app.primary.sim.lookup_person(id)?;
                return Some(Transition::Multi(vec![
                    Transition::Pop,
                    Transition::ModifyState(Box::new(move |state, ctx, app| {
                        // Other states pretty much don't use info panels.
                        if let Some(ref mut s) = state.downcast_mut::<SandboxMode>() {
                            let mut actions = s.contextual_actions();
                            s.controls.common.as_mut().unwrap().launch_info_panel(
                                ctx,
                                app,
                                Tab::PersonTrips(id, BTreeMap::new()),
                                &mut actions,
                            );
                        }
                    })),
                ]));
            }
            'c' => {
                // This one gets more complicated. :)
                let c = app.primary.sim.lookup_car_id(idx)?;
                ID::Car(c)
            }
            't' => {
                let trip = TripID(idx);
                let person = app.primary.sim.trip_to_person(trip)?;
                return Some(Transition::Multi(vec![
                    Transition::Pop,
                    Transition::ModifyState(Box::new(move |state, ctx, app| {
                        // Other states pretty much don't use info panels.
                        if let Some(ref mut s) = state.downcast_mut::<SandboxMode>() {
                            let mut actions = s.contextual_actions();
                            s.controls.common.as_mut().unwrap().launch_info_panel(
                                ctx,
                                app,
                                Tab::PersonTrips(person, OpenTrip::single(trip)),
                                &mut actions,
                            );
                        }
                    })),
                ]));
            }
            _ => {
                return None;
            }
        },
        Err(_) => {
            return None;
        }
    };
    if let Some(pt) = app.primary.canonical_point(id.clone()) {
        println!("Warping to {:?}", id);
        Some(Transition::Replace(Warping::new(
            ctx,
            pt,
            Some(WARP_TO_CAM_ZOOM),
            Some(id),
            &mut app.primary,
        )))
    } else {
        None
    }
}
