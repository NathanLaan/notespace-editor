//!
//! Notespace-Editor
//!
//! User configuration settings.
//!

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SETTINGS_FILE: &str = "notespace_settings.toml";

///
/// User configuration settings.
///
#[derive(Serialize, Deserialize)]
pub(crate) struct AppConfiguration {
    pub window_x: f32,
    pub window_y: f32,
    pub window_w: f32,
    pub window_h: f32,
    pub scale_factor: f64,
    pub window_theme_name: String,
    pub syntax_theme_name: String,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            window_x: 100.0,
            window_y: 100.0,
            window_w: 800.0,
            window_h: 600.0,
            scale_factor: 1.0,
            window_theme_name: iced::Theme::default().to_string(),
            syntax_theme_name: iced::highlighter::Theme::SolarizedDark.to_string(),
        }
    }
}

impl AppConfiguration {
    ///
    /// Shortcut function for use when the application loads.
    ///
    pub(crate) fn get_window_size(&self) -> iced::Size {
        // let w = self.window_w * self.scale_factor as f32;
        // let h = self.window_h * self.scale_factor as f32;
        // iced::Size::new(w, h)
        iced::Size::new(self.window_w, self.window_h)
    }

    ///
    /// Shortcut function for use when the application loads.
    ///
    pub(crate) fn get_window_position(&self) -> iced::window::Position {
        iced::window::Position::Specific(iced::Point {
            x: self.window_w,
            y: self.window_h,
        })
    }

    ///
    /// Returns the configuration file path:
    ///
    /// `dirs::config_dir() + SETTINGS_FILE`
    ///
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(SETTINGS_FILE)
    }

    ///
    /// Attempts to load `AppConfiguration` from the configuration file.
    ///
    /// Returns `AppConfiguration::default()` if no configuration file is found.
    ///
    pub fn load() -> Self {
        let path = Self::path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<AppConfiguration>(&content) {
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::path();
        println!("saving config to {:?}", &path);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(toml) = toml::to_string_pretty(self) {
            let _ = fs::write(path, toml);
        }
    }
}
