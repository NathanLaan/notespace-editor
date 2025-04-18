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
    pub scale_factor: f64,
    pub active_theme: Theme,
    pub text_content: Content,
    pub error: Option<AppIOError>,
    pub file_path: Option<PathBuf>,
    pub font_monospaced: Option<Font>,
    pub(crate) file_dirty: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scale_factor: 1.5,
            active_theme: Theme::Dark,
            text_content: Content::default(),
            error: None,
            file_path: None,
            font_monospaced: None,
            file_dirty: false,
        }
    }
}
