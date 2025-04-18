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
use fa_iced::load_font_fontawesome;

use iced::settings::Settings;
use app_main::AppMain;
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    //
    // Load the Font Awesome fonts.
    //
    load_font_fontawesome();

    //
    // Setup and run the app.
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = iced::Size::new(1200.0,1024.0);
    iced::application(AppMain::title, AppMain::update, AppMain::view)
        .window(window_settings)
        .scale_factor(AppMain::scale_factor)
        .theme(AppMain::theme)
        .run()
}
