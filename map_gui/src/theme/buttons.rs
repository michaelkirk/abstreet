use crate::colors::{ButtonColorScheme, ColorScheme};
use widgetry::{ButtonBuilder, ButtonState, ScreenDims};

pub trait Buttons<'a> {
    fn btn_primary_dark(&self) -> ButtonBuilder<'a>;
    fn btn_primary_dark_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_primary_dark().label_text(text)
    }
    fn btn_primary_dark_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_primary_dark().image_path(image_path))
    }
    fn btn_primary_dark_image_text(&self, image_path: &'a str, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_primary_dark()
            .label_text(text)
            .image_path(image_path)
            .image_dims(ScreenDims::square(18.0))
    }

    fn btn_secondary_dark(&self) -> ButtonBuilder<'a>;
    fn btn_secondary_dark_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_secondary_dark().label_text(text)
    }
    fn btn_secondary_dark_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_secondary_dark().image_path(image_path))
    }
    fn btn_secondary_dark_image_text(
        &self,
        image_path: &'a str,
        text: &'a str,
    ) -> ButtonBuilder<'a> {
        self.btn_secondary_dark()
            .label_text(text)
            .image_path(image_path)
            .image_dims(ScreenDims::square(18.0))
    }

    fn btn_primary_light(&self) -> ButtonBuilder<'a>;
    fn btn_primary_light_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_primary_light().label_text(text)
    }
    fn btn_primary_light_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_primary_light().image_path(image_path))
    }
    fn btn_primary_light_image_text(
        &self,
        image_path: &'a str,
        text: &'a str,
    ) -> ButtonBuilder<'a> {
        self.btn_primary_light()
            .label_text(text)
            .image_path(image_path)
            .image_dims(ScreenDims::square(18.0))
    }

    fn btn_secondary_light(&self) -> ButtonBuilder<'a>;
    fn btn_secondary_light_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_secondary_light().label_text(text)
    }
    fn btn_secondary_light_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_secondary_light().image_path(image_path))
    }
    fn btn_secondary_light_image_text(
        &self,
        image_path: &'a str,
        text: &'a str,
    ) -> ButtonBuilder<'a> {
        self.btn_secondary_light()
            .label_text(text)
            .image_path(image_path)
            .image_dims(ScreenDims::square(18.0))
    }

    fn btn_plain_dark(&self) -> ButtonBuilder<'a>;
    fn btn_plain_dark_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_plain_dark().label_text(text)
    }
    fn btn_plain_dark_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_plain_dark().image_path(image_path))
    }

    fn btn_plain_light(&self) -> ButtonBuilder<'a>;
    fn btn_plain_light_text(&self, text: &'a str) -> ButtonBuilder<'a> {
        self.btn_plain_light().label_text(text)
    }
    fn btn_plain_light_image(&self, image_path: &'a str) -> ButtonBuilder<'a> {
        image_padding(self.btn_plain_light().image_path(image_path))
    }

    // Specific UI Elements

    /// title: name of previous screen, which you'll return to
    fn btn_back_light(&self, title: &'a str) -> ButtonBuilder<'a>;

    /// title: name of previous screen, which you'll return to
    fn btn_back_dark(&self, title: &'a str) -> ButtonBuilder<'a>;
}

impl<'a> Buttons<'a> for ColorScheme {
    fn btn_primary_dark(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_primary_dark;
        plain_builder(colors).outline(2.0, colors.outline, ButtonState::Default)
    }

    fn btn_secondary_dark(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_secondary_dark;
        plain_builder(colors).outline(2.0, colors.outline, ButtonState::Default)
    }

    fn btn_plain_dark(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_secondary_dark;
        plain_builder(colors)
    }

    fn btn_primary_light(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_primary_light;
        plain_builder(colors).outline(2.0, colors.outline, ButtonState::Default)
    }

    fn btn_secondary_light(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_secondary_light;
        plain_builder(colors).outline(2.0, colors.outline, ButtonState::Default)
    }

    fn btn_plain_light(&self) -> ButtonBuilder<'a> {
        let colors = &self.btn_secondary_light;
        plain_builder(colors)
    }

    // specific UI elements

    fn btn_back_light(&self, title: &'a str) -> ButtonBuilder<'a> {
        back_button(self.btn_plain_light(), title)
    }

    fn btn_back_dark(&self, title: &'a str) -> ButtonBuilder<'a> {
        back_button(self.btn_plain_dark(), title)
    }
}

fn plain_builder<'a>(color_scheme: &ButtonColorScheme) -> ButtonBuilder<'a> {
    ButtonBuilder::new()
        .label_color(color_scheme.fg, ButtonState::Default)
        .label_color(color_scheme.fg_disabled, ButtonState::Disabled)
        .image_color(color_scheme.fg, ButtonState::Default)
        .image_color(color_scheme.fg_disabled, ButtonState::Disabled)
        .bg_color(color_scheme.bg, ButtonState::Default)
        .bg_color(color_scheme.bg_hover, ButtonState::Hover)
        .bg_color(color_scheme.bg_disabled, ButtonState::Disabled)
}

fn image_padding<'a>(builder: ButtonBuilder<'a>) -> ButtonBuilder<'a> {
    builder.padding(4)
}

fn back_button<'a>(builder: ButtonBuilder<'a>, title: &'a str) -> ButtonBuilder<'a> {
    // DESIGN REVIEW: this button seems absurdly large
    builder
        .image_path("system/assets/pregame/back.svg")
        .label_text(title)
        .padding_left(8.0)
        .font_size(30)
}
