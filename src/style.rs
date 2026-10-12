//! Shift's look, taken entirely from the active Colony palette, so a theme,
//! an accent or high contrast picked in Preferences reaches every widget.

use crate::app::Shift;
use colony_ui::theme::{active_palette, effective_accent};
use iced::widget::{button, container};
use iced::{Border, Theme};

/// The iced theme built from the active palette, so the widgets Shift does
/// not style itself (scrollbars, the drop-down list) match the rest.
pub fn theme(_state: &Shift) -> Theme {
    let palette = active_palette();
    Theme::custom(
        "Colony",
        iced::theme::Palette {
            background: palette.bg_primary,
            text: palette.text_primary,
            primary: effective_accent(),
            success: palette.success,
            warning: palette.warning,
            danger: palette.error,
        },
    )
}

/// The buttons of the control bar and of the start screen.
pub fn action(_theme: &Theme, status: button::Status) -> button::Style {
    let palette = active_palette();
    let (background, text_color) = match status {
        button::Status::Active => (palette.btn_default, palette.text_primary),
        button::Status::Hovered => (palette.btn_hover, palette.text_primary),
        button::Status::Pressed => (palette.btn_pressed, palette.text_primary),
        button::Status::Disabled => (palette.btn_default, palette.text_dimmest),
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border::default().rounded(6),
        ..Default::default()
    }
}

/// The control bar, the details bar and the status line.
pub fn bar(_theme: &Theme) -> container::Style {
    let palette = active_palette();
    container::Style {
        background: Some(palette.bg_card.into()),
        border: Border {
            color: palette.border_subtle,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

/// Behind the image: the deepest surface of the palette.
pub fn viewer(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(active_palette().bg_sidebar.into()),
        ..Default::default()
    }
}

pub fn tooltip(_theme: &Theme) -> container::Style {
    let palette = active_palette();
    container::Style {
        background: Some(palette.bg_card_hover.into()),
        text_color: Some(palette.text_primary),
        border: Border {
            color: palette.border_subtle,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    /// Every text size goes through `sz`, so both text-size preferences reach
    /// it, and every colour comes from the palette, so it is right on every
    /// theme (design/typography.md, design/settings-page.md). Reading the
    /// sources is the only way to catch a literal before it reaches a screen.
    #[test]
    fn no_raw_text_size_and_no_literal_colour() {
        let sources = [
            ("app.rs", include_str!("app.rs")),
            ("preferences.rs", include_str!("preferences.rs")),
            ("style.rs", include_str!("style.rs")),
        ];
        let literal_colour = concat!("Color::", "from_rgb");

        for (file, source) in sources {
            for (number, line) in source.lines().enumerate() {
                let raw_size = line.match_indices(".size(").any(|(at, call)| {
                    line[at + call.len()..].starts_with(|c: char| c.is_ascii_digit())
                });
                assert!(
                    !raw_size && !line.contains(literal_colour),
                    "{file}:{}: {line}",
                    number + 1
                );
            }
        }
    }
}
