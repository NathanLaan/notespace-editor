//!
//! Notespace-Editor
//!
//! Statusbar for main app window.
//!

use std::fmt::format;
use std::path::Path;
use iced::{widget::{horizontal_space, text, Text, row}, Font, Renderer, Theme};
use super::app_message::AppMessage;
use super::app_state::AppState;
use crate::app_const::{UI_CONTROL_PADDING, UI_CONTROL_SPACING, UI_STATUSBAR_TEXT_SIZE};

pub struct AppStatusbar;

impl AppStatusbar {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn view<'a>(&self, app_state: &'a AppState) -> iced::Element<'a, AppMessage> {

        //
        // TODO: When user clicks on file_path, open the containing folder
        //

        //
        // Create text() control with the file_path.
        // Handle paths that crash with and_then().
        //
        let file_path_display: Text<Theme, Renderer> = match &app_state
            .error {
                Some(e) => Text::new(format!("Error: {}", e.to_string())),
                None => match app_state
                    .file_path
                    .as_deref()
                    .and_then(Path::to_str) {
                    Some(file_path) =>
                        text(file_path)
                            .font(app_state.font_monospaced.unwrap_or(Font::MONOSPACE))
                            .size(UI_STATUSBAR_TEXT_SIZE),
                    None => text(""),
                }
            };


        let cursor_position = {
            let (l,c) = app_state.text_content.cursor_position();
            text(format!("[ {}:{} ]", l, c))
                .font(app_state.font_monospaced.unwrap_or(Font::MONOSPACE))
        };

        row![
            file_path_display,
            horizontal_space(),
            cursor_position,
        ]
            .spacing(UI_CONTROL_SPACING)
            .padding(UI_CONTROL_PADDING)
            .into()
    }
}