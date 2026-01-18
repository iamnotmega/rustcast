use iced::Theme;
use iced::border::Radius;
use iced::{Background, Border, Color, widget::text_input};

/// Helper: mix base color with white (simple “tint”)
pub fn tint(mut c: Color, amount: f32) -> Color {
    c.r = c.r + (1.0 - c.r) * amount;
    c.g = c.g + (1.0 - c.g) * amount;
    c.b = c.b + (1.0 - c.b) * amount;
    c
}

/// Helper: apply alpha
pub fn with_alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}

pub fn rustcast_text_input_style(
    theme: &Theme,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + '_ {
    move |_, status| {
        let palette = theme.palette();
        let base_bg = palette.background;
        let surface = with_alpha(tint(base_bg, 0.06), 1.0);

        let (border_color, border_width) = match status {
            text_input::Status::Focused { .. } => (palette.text, 1.2),
            text_input::Status::Hovered => (palette.text, 1.0),
            text_input::Status::Active => (palette.text, 0.9),
            text_input::Status::Disabled => (palette.text, 0.8),
        };

        text_input::Style {
            background: Background::Color(surface),
            border: Border {
                color: border_color,
                width: border_width,
                radius: Radius::new(12.0),
            },
            icon: palette.text,
            placeholder: palette.text,
            value: palette.text,
            selection: palette.text,
        }
    }
}
