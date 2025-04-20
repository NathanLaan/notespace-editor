//!
//! Notespace-Editor
//!
//! User configuration settings.
//!

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

///
/// User configuration settings.
///
#[derive(Serialize, Deserialize)]
pub(crate) struct AppConfiguration {
    pub window_width: u32,
    pub window_height: u32,
    pub scale_factor: f32,
    pub window_theme_name: String,
    pub syntax_theme_name: String,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            scale_factor: 1.0,
            window_theme_name: iced::Theme::default().to_string(),
            syntax_theme_name: iced::highlighter::Theme::SolarizedDark.to_string(),
        }
    }
}

impl AppConfiguration {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("settings.toml")
    }

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
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(toml) = toml::to_string_pretty(self) {
            let _ = fs::write(path, toml);
        }
    }
}