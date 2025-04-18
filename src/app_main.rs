//!
//! Notespace-Editor
//!
//! The Iced `Application` implementation.
//!

use iced::Font;
use iced::{Element, Task, Theme, Length};
use iced::advanced::text::Highlight;
use iced::widget::{container, column, text_editor};
use iced::highlighter::{self, Highlighter};
use super::app_toolbar::AppToolbar;
use super::app_message::AppMessage;
use super::app_state::AppState;
use super::app_statusbar::AppStatusbar;
use super::app_io::{async_open_file_from_dialog, async_save_file_to_path};
use rust_i18n::t;

///
/// The top-level Iced Application component.
///
pub struct AppMain {
    app_state: AppState,
    toolbar: AppToolbar,
    statusbar: AppStatusbar,
    scrollable_editor: iced::widget::scrollable::Id,
}

///
/// Default implementation.
///
impl Default for AppMain {
    fn default() -> Self {
        let app_state = AppState::default();
        Self {
            app_state,
            toolbar: AppToolbar::new(),
            statusbar: AppStatusbar::new(),
            scrollable_editor: iced::widget::scrollable::Id::unique(),
        }
    }
}

impl AppMain {
    // ///
    // /// Iced Application Traits:
    // ///
    // type Executor = executor::Default;
    // type Message = AppMessage;
    // type Theme = Theme;
    // type Flags = ();

    ///
    /// Constructor.
    ///
    pub fn new() -> (Self, Task<AppMessage>) {
        let app = AppMain::default();
        (
            app,
            Task::none(),
        )
    }

    ///
    /// Iced function to get the window title.
    ///
    pub fn title(&self) -> String {
        t!("app_name").to_string()
    }

    ///
    /// Iced function to handle messages.
    ///
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::TextEdited(action) => {
                // reset error
                self.app_state.error = None;
                self.app_state.file_dirty = self.app_state.file_dirty || action.is_edit();
                self.app_state.file_content.perform(action);
                Task::none()
            },
            AppMessage::OpenFileFromDialog => {
                Task::perform(
                    async_open_file_from_dialog(),
                    AppMessage::FileOpened)
            },
            AppMessage::FileOpened(Ok((file_path, content))) => {
                self.app_state.file_path = Some(file_path);
                self.app_state.file_content = text_editor::Content::with_text(content.as_ref());
                Task::none()
            },
            AppMessage::FileOpened(Err(error)) => {
                self.app_state.error = Some(error);
                Task::none()
            },
            AppMessage::NewFile => {
                if self.app_state.file_dirty {
                    //
                    // TODO: Show error dialog.
                    //
                    println!("File Modified");
                } else {
                    self.app_state.file_path = None;
                    self.app_state.file_content = text_editor::Content::new();
                }
                Task::none()
            },
            AppMessage::SaveFile => {
                Task::perform(
                    async_save_file_to_path(
                        self.app_state.file_path.clone(),
                        self.app_state.file_content.text().clone(),
                    ),
                AppMessage::FileSaved)
            },
            AppMessage::FileSaved(Ok(file_name)) => {
                self.app_state.file_path = Some(file_name);
                self.app_state.file_dirty = false;
                Task::none()
            },
            AppMessage::FileSaved(Err(error)) => {
                self.app_state.error = Some(error);
                Task::none()
            },
            AppMessage::UpdateLanguage(str) => {
                rust_i18n::set_locale(str.as_ref());
                Task::none()
            },
            AppMessage::UpdateWindowTheme(theme) => {
                self.app_state.window_theme = theme;
                Task::none()
            },
            AppMessage::UpdateSyntaxTheme(theme) => {
                self.app_state.syntax_theme = theme;
                Task::none()
            },
            AppMessage::UpdateScale(value) => {
                self.app_state.scale_factor = value;
                Task::none()
            },
        }
    }

    ///
    /// Iced function to render the view.
    ///
    pub fn view(&self) -> Element<'_, AppMessage> {
        let ext = self.app_state
            .file_path
            .as_ref()
            .and_then(|path|path.extension()?.to_str())
            .unwrap_or("rs")
            .to_string();
        let editor = text_editor(&self.app_state.file_content)
            .highlight(ext.as_str(), self.app_state.syntax_theme)
            .on_action(AppMessage::TextEdited)
            .height(Length::Fill)
            .font(self.app_state.font_monospaced.unwrap_or(Font::MONOSPACE));

        //
        // TODO: Fix for iced 0.13 scrolling.
        //
        //scrollable(column![editor]).into();
        // let scrollable = Scrollable::new(self.scrollable_editor.clone())
        //     .height(Length::Fill)
        //     .push(editor);

        container(column![
            self.toolbar.view(&self.app_state),
            editor,
            self.statusbar.view(&self.app_state),
        ])
            .padding(5)
            .into()
    }

    ///
    /// Iced function to get the Theme.
    ///
    pub fn theme(&self) -> Theme {
        self.app_state.window_theme.clone()
    }

    ///
    /// Iced function to get the view scale_factor.
    ///
    pub fn scale_factor(&self) -> f64 {
        self.app_state.scale_factor.clone()
    }

}