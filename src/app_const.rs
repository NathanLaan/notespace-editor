//!
//! Notespace-Editor
//!
//! Constants for the application.
//!

///
/// App name
///
pub(crate) const APP_NAME: &str = "Notespace-Editor";

///
/// Horizontal spacing for controls.
///
pub(crate) const UI_CONTROL_SPACING: u16 = 5;

///
/// Spacing around controls.
///
pub(crate) const UI_CONTROL_PADDING: u16 = 5;

///
/// Spacing around tooltips.
///
pub(crate) const UI_TOOLTIP_PADDING: u16 = 5;
pub(crate) const UI_TOOLTIP_RADIUS: u16 = 5;

///
/// The size of text in the statusbar.
///
pub(crate) const UI_STATUSBAR_TEXT_SIZE: u16 = 12;

///
/// The size of toolbar buttons.
///
pub(crate) const UI_TOOLBAR_BUTTON_SIZE: u16 = 30;

///
/// The size of toolbar button icons.
///
pub(crate) const UI_TOOLBAR_ICON_SIZE: u16 = 16;

pub(crate) const FONT_MONOSPACED_BYTES: &[u8] = include_bytes!("../fonts/FiraCode-Regular.ttf");
//pub const FONT_MONOSPACED: iced::Font = iced::Font(FONT_MONOSPACED_BYTES);
