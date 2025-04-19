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
pub struct AppStyle {
    pub color: Color,
}

impl AppStyle {
    //type Style = Theme;

    fn appearance(&self, _style: &container::Style) -> container::Style {
        container::Style {
            background: Some(self.color.into()),
            border: Border::default(),
            text_color: None,
            shadow: Default::default(),
        }
    }


    pub(crate) fn tooltip_style(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.weak.color.into()),
            border: border::rounded(UI_TOOLTIP_RADIUS),
            ..Style::default()
        }
    }
    
    pub(crate) fn toolbar_theme(theme: &Theme) -> Style {
        let palette = theme.extended_palette();
        Style {
            background: Some(palette.background.strong.color.into()),
            ..Style::default()
        }
    }

}
