#![allow(dead_code)]
// Link-style button: looks like a hyperlink (blue text, no background/border).
//
// Usage: `button(text("Click")).style(link_style)`

use iced::widget::button;
use iced::{Background, Border, Color};

/// Canonical link/URL color used across the UI.
pub const LINK_COLOR: Color = Color::from_rgb(0.3, 0.5, 1.0);

pub fn link_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let color = Color::from_rgb(0.29, 0.62, 1.0); // #4A9EFF
    let dark = Color::from_rgb(0.22, 0.50, 0.88); // pressed tint

    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: color,
            border: Border::default(),
            shadow: Default::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.29, 0.62, 1.0, 0.1))),
            text_color: color,
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Default::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.29, 0.62, 1.0, 0.15))),
            text_color: dark,
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Default::default(),
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color::from_rgba(0.29, 0.62, 1.0, 0.4),
            border: Border::default(),
            shadow: Default::default(),
        },
    }
}

/// Small cancel/remove button: subtle background with red hover tint.
///
/// Usage: `button(text("x")).style(cancel_btn_style)`
pub fn cancel_btn_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let text_color = Color::from_rgb(0.6, 0.6, 0.6);
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.15))),
            text_color,
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Default::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.9, 0.3, 0.3, 0.25))),
            text_color: Color::from_rgb(0.95, 0.3, 0.3),
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Default::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.9, 0.3, 0.3, 0.35))),
            text_color: Color::from_rgb(0.85, 0.2, 0.2),
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Default::default(),
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color::from_rgba(0.5, 0.5, 0.5, 0.3),
            border: Border::default(),
            shadow: Default::default(),
        },
    }
}
