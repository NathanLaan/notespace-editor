//!
//! Notespace-Editor
//!
//! The `main()` function.
//!

mod app_main;
mod app_const;
mod app_message;
mod app_state;
mod app_statusbar;
mod app_io;
mod app_toolbar;
mod app_style;
mod app_configuration;
mod keyboard;

use app_main::AppMain;
use fa_iced::load_font_fontawesome_ttf;
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
    // TODO: Load app configuration here to get window size and location
    //

    //
    // Initialize and run the app.
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = iced::Size::new(1200.0,1024.0);
    iced::application(AppMain::title, AppMain::update, AppMain::view)
        .subscription(AppMain::subscription)
        .window(window_settings)
        .scale_factor(AppMain::scale_factor)
        .theme(AppMain::theme)
        .run()
    //AppMain::run(iced::Settings::default())
}
