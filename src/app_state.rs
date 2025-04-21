//!
//! Notespace-Editor
//!
//! Iced application state.
//!
//! Canonical reference is stored in `AppMain.app_state`.
//!

use std::path::PathBuf;
use iced::Theme;
use iced::widget::text_editor::Content;
use iced::font::Font;
use crate::app_io::AppIOError;

pub struct AppState {
    pub(crate) scale_factor: f64,
    pub(crate) window_theme: Theme,
    pub(crate) syntax_theme: iced::highlighter::Theme,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) file_dirty: bool,
    pub(crate) file_content: Content,
    pub(crate) error: Option<AppIOError>,
    pub(crate) font_monospaced: Option<Font>,
    pub(crate) app_configuration_changed:bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scale_factor: 1.5,
            window_theme: Theme::Dark,
            syntax_theme: iced::highlighter::Theme::SolarizedDark,
            file_path: None,
            file_dirty: false,
            file_content: Content::default(),
            error: None,
            font_monospaced: None,
            app_configuration_changed: false,
        }
    }
}
