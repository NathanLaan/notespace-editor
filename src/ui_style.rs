//!
//! Notespace-Editor
//!
//! Style/Appearance related components.
//!
//! References:
//! - https://github.com/iced-rs/iced/discussions/1266
//!

use super::ui_const::{UI_LOGIN_BUTTON_RADIUS, UI_TOOLTIP_RADIUS};
use iced::widget::button::Status;
use iced::widget::container::Style;
use iced::{Border, Color, Theme, border};

///
/// Style functions.
///
pub struct AppStyle;

impl AppStyle {
    ///
    /// Style for a container.
    ///
    /// Can be used to set border style.
    ///
    pub(crate) fn _color_container(color: Color, theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(color.into()),
            border: Border::default(),
            text_color: palette.primary.base.color.into(),
            shadow: Default::default(),
        }
    }

    /// Style for a `text()` control.
    pub(crate) fn text_style_secondary_base(theme: &Theme) -> iced::widget::text::Style {
        let palette = theme.extended_palette();
        iced::widget::text::Style {
            color: Some(palette.secondary.base.color.into()),
        }
    }

    /// Style for a `text()` control.
    pub(crate) fn text_style_secondary_weak(theme: &Theme) -> iced::widget::text::Style {
        let palette = theme.extended_palette();
        iced::widget::text::Style {
            color: Some(palette.secondary.weak.color.into()),
        }
    }

    /// Style for a `text()` control.
    pub(crate) fn text_style_primary_weak(theme: &Theme) -> iced::widget::text::Style {
        let palette = theme.extended_palette();
        iced::widget::text::Style {
            color: Some(palette.primary.weak.color.into()),
        }
    }

    ///
    /// Style for a tooltip, based on the `theme`.
    ///
    pub(crate) fn style_tooltip(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.weak.color.into()),
            border: border::rounded(UI_TOOLTIP_RADIUS),
            ..Style::default()
        }
    }

    /// A rounded [`Container`] with a background.
    pub(crate) fn style_modal_container(theme: &Theme) -> Style {
        let palette = theme.extended_palette();

        Style {
            background: Some(palette.background.strong.color.into()),
            border: border::rounded(5),
            ..Style::default()
        }
    }

    ///
    /// Style for a modal header, based on the `theme`.
    ///
    pub(crate) fn style_modal_header(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.weak.color.into()),
            border: border::rounded(5),
            ..Style::default()
        }
    }

    ///
    /// Style for a toolbar, based on the `theme`.
    ///
    pub(crate) fn style_toolbar(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.strong.color.into()),
            ..Style::default()
        }
    }

    ///
    /// Style for a numpad button, used for the login PIN entry.
    ///
    pub(crate) fn button_style_numpad(
        theme: &Theme,
        status: Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let background_color: Color = match status {
            Status::Active => palette.primary.strong.color,
            Status::Hovered => palette.primary.weak.color,
            Status::Pressed => palette.primary.base.color,
            Status::Disabled => palette.primary.weak.color,
        };
        iced::widget::button::Style {
            text_color: theme.palette().text,
            background: Some(background_color.into()),
            border: border::rounded(UI_LOGIN_BUTTON_RADIUS),
            ..iced::widget::button::Style::default()
        }
    }

    ///
    /// Style for an ok button.
    ///
    pub(crate) fn button_style_ok(theme: &Theme, status: Status) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let background_color: Color = match status {
            Status::Active => palette.success.strong.color,
            Status::Hovered => palette.success.weak.color,
            Status::Pressed => palette.success.base.color,
            Status::Disabled => palette.success.weak.color,
        };
        iced::widget::button::Style {
            text_color: theme.palette().text,
            background: Some(background_color.into()),
            ..iced::widget::button::Style::default()
        }
    }

    ///
    /// Style for an ok button.
    ///
    pub(crate) fn button_style_secondary(
        theme: &Theme,
        status: Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let background_color: Color = match status {
            Status::Active => palette.secondary.strong.color,
            Status::Hovered => palette.secondary.weak.color,
            Status::Pressed => palette.secondary.base.color,
            Status::Disabled => palette.secondary.weak.color,
        };
        iced::widget::button::Style {
            text_color: theme.palette().text,
            background: Some(background_color.into()),
            ..iced::widget::button::Style::default()
        }
    }

    ///
    /// Style for a cancel button.
    ///
    pub(crate) fn button_style_danger(
        theme: &Theme,
        status: Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let background_color: Color = match status {
            Status::Active => palette.danger.strong.color,
            Status::Hovered => palette.danger.weak.color,
            Status::Pressed => palette.danger.base.color,
            Status::Disabled => palette.danger.weak.color,
        };
        iced::widget::button::Style {
            text_color: theme.palette().text,
            background: Some(background_color.into()),
            ..iced::widget::button::Style::default()
        }
    }

    ///
    /// Style for a cancel button.
    ///
    pub(crate) fn button_style_primary(
        theme: &Theme,
        status: Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let background_color: Color = match status {
            Status::Active => palette.primary.strong.color,
            Status::Hovered => palette.primary.weak.color,
            Status::Pressed => palette.primary.base.color,
            Status::Disabled => palette.primary.weak.color,
        };
        iced::widget::button::Style {
            text_color: theme.palette().text,
            background: Some(background_color.into()),
            ..iced::widget::button::Style::default()
        }
    }
}
