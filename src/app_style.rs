//!
//! Notespace-Editor
//!
//! Style/Appearance related components.
//!

use iced::widget::container;
use iced::{Border, Color};

///
/// Allows an `iced::widget::Container` to have a colored background.
///
pub struct ColoredBackground(pub Color);
impl ColoredBackground {
    //type Style = Theme;

    fn appearance(&self, _style: &container::Style) -> container::Style {
        container::Style {
            background: Some(self.0.into()),
            border: Border::default(),
            text_color: None,
            shadow: Default::default(),
        }
    }

}
