//!
//! Notespace-Editor
//!
//! The `main()` function.
//!

mod app_configuration;
mod app_const;
mod app_io;
mod app_message;
mod app_state;
mod controls;
mod keyboard;
mod main_window;
mod ui_const;
mod ui_style;
mod ui_util;

use crate::app_configuration::AppConfiguration;
use fa_iced::load_font_fontawesome_ttf;
use main_window::AppMain;

extern crate rust_i18n;

//
// Load i18n translations.
//
rust_i18n::i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    //
    // Load the Font Awesome fonts.
    //
    load_font_fontawesome_ttf();

    //
    // TODO: Load app configuration.
    //
    // TODO: Load the `scale_factor` and multiply the size before setting.
    //
    let app_configuration = AppConfiguration::load();
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = app_configuration.get_window_size();
    window_settings.position = app_configuration.get_window_position();

    //
    // Initialize and run the app.
    //
    iced::application(AppMain::title, AppMain::update, AppMain::view)
        .subscription(AppMain::subscription)
        .window(window_settings)
        .scale_factor(AppMain::scale_factor)
        .theme(AppMain::theme)
        .run()
}
