//!
//! Notespace-Editor
//!
//! Style/Appearance related components.
//!
//! References:
//! - https://github.com/iced-rs/iced/discussions/1266
//!

use iced::widget::container;
use iced::{border, Border, Color, Theme};
use iced::widget::container::Style;
use super::app_const::{UI_TOOLTIP_RADIUS};

///
/// Allows an `iced::widget::Container` to have a colored background.
///
pub struct AppStyle;

impl AppStyle {


    pub(crate) fn _color_container(color: Color, theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(color.into()),
            border: Border::default(),
            text_color: palette.primary.base.color.into(),
            shadow: Default::default(),
        }
    }


    ///
    /// Style for a tooltip, based on the `theme`.
    ///
    pub(crate) fn tooltip_style(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.weak.color.into()),
            border: border::rounded(UI_TOOLTIP_RADIUS),
            ..Style::default()
        }
    }

    ///
    /// Style for a toolbar, based on the `theme`.
    ///
    pub(crate) fn toolbar_theme(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.strong.color.into()),
            ..Style::default()
        }
    }

}
