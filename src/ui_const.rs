//!
//! Smartcard-Iced-App
//!
//! Constants for the application.
//!

///
/// Shortcut for: `Arc::new(Mutex::new(obj))`.
///
#[macro_export]
macro_rules! arc_mutex {
    ($obj:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($obj))
    };
}

///
/// Horizontal spacing for controls.
///
pub const UI_CONTROL_SPACING: u16 = 5;

///
/// Spacing around controls.
///
pub const UI_CONTROL_PADDING: u16 = 5;

pub const UR_LEFT_SIDEBAR_PADDING: iced::Padding = iced::Padding {
    top: 0.0,
    bottom: 5.0,
    left: 5.0,
    right: 5.0,
};

///
/// The size of toolbar buttons.
///
pub const UI_TOOLBAR_BUTTON_HEIGHT_LARGE: u16 = 60;

///
///
///
pub const UI_TOOLBAR_ICON_FONT_SIZE_LARGE: u16 = 32;

///
/// The size of toolbar buttons.
///
pub const UI_TOOLBAR_BUTTON_HEIGHT_SMALL: u16 = 20;

///
/// The size of toolbar button icons.
///
pub const UI_LOGIN_PIN_ICON_SIZE: u16 = 46;

//pub const UI_TOPBAR_ICON_FONT_SIZE: u16 = 32; //24;

pub const UI_MODAL_LABEL_SIZE: u16 = 150;

pub(crate) const UI_TOOLTIP_PADDING: u16 = 5;
pub(crate) const UI_TOOLTIP_RADIUS: u16 = 5;

pub(crate) const UI_LOGIN_BUTTON_RADIUS: u16 = 0;

pub const UI_USER_INFO_FONT_SIZE: u16 = 16;

pub(crate) const UI_COL_WIDTH_USERNAME: u16 = 3;
pub(crate) const UI_COL_WIDTH_PASSWORD: u16 = 2;
pub(crate) const UI_COL_WIDTH_SMARTCARD: u16 = 7;
