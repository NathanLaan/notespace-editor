//!
//! Notespace-Editor
//!
//! The Iced `Application` implementation.
//!

use iced::Font;
use iced::{Element, Task, Theme, Length};
use iced::event;
use iced::widget::{stack, center, horizontal_space, mouse_area, opaque};
use iced::widget::{container, column, row, text_editor};
use iced::keyboard::Key::Character;
use crate::controls::toolbar::AppToolbar;
use super::app_message::AppMessage;
use super::app_state::AppState;
use crate::controls::statusbar::AppStatusbar;
use super::app_configuration::AppConfiguration;
use super::app_io::{async_open_file_from_dialog, async_save_file_to_path};
use rust_i18n::t;

///
/// The top-level Iced Application component.
///
pub struct AppMain {
    app_state: AppState,
    toolbar: AppToolbar,
    statusbar: AppStatusbar,
    app_configuration: AppConfiguration,
    show_app_configuration_modal: bool,
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
            app_configuration: AppConfiguration::default(),
            show_app_configuration_modal: false,
        }
    }
}

impl AppMain {
    // ///
    // /// Iced Application Traits:
    // ///
    // type Executor = iced::executor::Default;
    // type Message = AppMessage;
    // type Theme = Theme;
    // type Flags = ();
    // //type State = AppState;

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
    /// Iced function to handle messages.
    ///
    pub(crate) fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::TextEdited(action) => {
                // reset error
                self.app_state.error = None;
                self.app_state.file_dirty = self.app_state.file_dirty || action.is_edit();
                self.app_state.file_content.perform(action);
                Task::none()
            },
            AppMessage::OpenFileFromDialog => {
                self.open_file()
            },
            AppMessage::FileOpened(Ok((file_path, content))) => {
                self.app_state.file_dirty = false;
                self.app_state.file_path = Some(file_path);
                self.app_state.file_content = text_editor::Content::with_text(content.as_ref());
                Task::none()
            },
            AppMessage::FileOpened(Err(error)) => {
                self.app_state.error = Some(error);
                Task::none()
            },
            AppMessage::NewFile => {
                self.new_file();
                Task::none()
            },
            AppMessage::SaveFile => {
                self.save_file()
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
            AppMessage::EventOccurred(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                                                key,
                                                                modifiers,
                                                                ..
                                                            })) => {
                let mut task = Task::none();
                //
                // TODO: Check Platform!
                //
                if (modifiers.control() || modifiers.command())
                    && !modifiers.shift()
                    && !modifiers.alt() {
                    println!("KeyPressed: {:?} {:?}", key, modifiers);
                    task = match key.as_ref() {
                        Character("s") => {
                            println!("SAVE!: {:?} {:?}", key, modifiers);
                            self.save_file()
                        }
                        Character("o") => {
                            self.open_file()
                        }
                        Character("n") => {
                            self.new_file();
                            Task::none()
                        }
                        _ => Task::none()
                    }
                }
                //
                // TODO: For situations where we track multi-key presses.
                //
                //let last_key = Some(key);
                task
            },
            AppMessage::EventOccurred(iced::Event::Mouse(_)) => {Task::none()},
            AppMessage::EventOccurred(iced::Event::Window(_)) => {Task::none()},
            AppMessage::EventOccurred(iced::Event::Touch(_)) => {Task::none()},
            AppMessage::EventOccurred(iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(_))) => {Task::none()},
            AppMessage::EventOccurred(iced::Event::Keyboard(iced::keyboard::Event::KeyReleased { .. })) => {Task::none()},
            AppMessage::WindowResized(w, h) => {
                self.app_configuration.window_w = w;
                self.app_configuration.window_h = h;
                self.app_configuration.save();
                Task::none()
            },
            AppMessage::SaveAppConfiguration => {
                if self.app_state.app_configuration_changed {
                    self.app_configuration.save();
                    self.app_state.app_configuration_changed = false;
                }
                Task::none()
            },
            AppMessage::TabPressed => {
                Task::none()
            },
            AppMessage::FocusChanged(id) => {
                Task::none()
            },
            AppMessage::OpenAppConfigurationModal => {
                self.show_app_configuration_modal = true;
                Task::none()
            },
            AppMessage::CloseAppConfigurationModal => {
                self.show_app_configuration_modal = false;
                Task::none()
            },
        }
    }

    fn save_file(&mut self) -> Task<AppMessage> {
        Task::perform(
            async_save_file_to_path(
                self.app_state.file_path.clone(),
                self.app_state.file_content.text().clone(),
            ),
            AppMessage::FileSaved)
    }

    fn open_file(&mut self) -> Task<AppMessage> {
        Task::perform(
            async_open_file_from_dialog(),
            AppMessage::FileOpened)
    }

    fn new_file(&mut self) {
        if self.app_state.file_dirty {
            //
            // TODO: Show error dialog.
            //
            println!("File Modified");
        } else {
            self.app_state.file_dirty = false;
            self.app_state.file_path = None;
            self.app_state.file_content = text_editor::Content::new();
        }
    }

    ///
    /// Iced function to get the window title.
    ///
    pub(crate) fn title(&self) -> String {
        t!("app_name").to_string()
    }

    pub(crate) fn window(&self) -> iced::window::Settings {
        println!("AppMain::window()");
        iced::window::Settings {
            size: iced::Size::new(self.app_configuration.window_w, self.app_configuration.window_h),
            ..iced::window::Settings::default()
        }
    }

    ///
    /// Iced function to render the view.
    ///
    pub(crate) fn view(&self) -> Element<'_, AppMessage> {
        let file_extension = self.app_state
            .file_path
            .as_ref()
            .and_then(|path|path.extension()?.to_str())
            .unwrap_or("md") // TODO: App config setting for default file extension.
            .to_string();
        let editor = text_editor(&self.app_state.file_content)
            .highlight(file_extension.as_str(), self.app_state.syntax_theme)
            .on_action(AppMessage::TextEdited)
            .font(self.app_state.font_monospaced.unwrap_or(Font::MONOSPACE));
        let scrollable_container = iced::widget::Scrollable::new(editor)
            .width(Length::Fill)
            .height(Length::Fill);
        //.style(iced::widget::container::bordered_box);

        //
        // [ TOOLBAR   ]
        // [ EDITOR    ]
        // [ STATUSBAR ]
        //
        let base_contents = container(column![
            self.toolbar.view(&self.app_state),
            scrollable_container,
            self.statusbar.view(&self.app_state),
        ])
            .padding(0);

        if self.show_app_configuration_modal {
            let modal_contents = container(
                column![
                    iced::widget::text("Settings"),
                    iced::widget::text("Setting 1"),
                    iced::widget::text("Setting 2"),
                    iced::widget::text("Setting 3"),
                    row![
                        horizontal_space(),
                        iced::widget::button(iced::widget::text("OK"))
                            .on_press(AppMessage::CloseAppConfigurationModal),
                    ]
                ]
                    .spacing(20),
            )
                .width(600)
                .padding(10)
                .style(container::rounded_box);

            AppMain::modal(base_contents,
                           modal_contents,
                           AppMessage::CloseAppConfigurationModal)
        } else {
            base_contents.into()
        }
    }

    ///
    /// Iced function to get the Theme.
    ///
    pub(crate) fn theme(&self) -> Theme {
        self.app_state.window_theme.clone()
    }

    ///
    /// Iced function to get the view scale_factor.
    ///
    pub(crate) fn scale_factor(&self) -> f64 {
        self.app_state.scale_factor.clone()
    }

    ///
    /// Iced function to handle subscriptions (async events).
    ///
    pub(crate) fn subscription(&self) -> iced::Subscription<AppMessage> {
        event::listen().map(AppMessage::EventOccurred)
        // iced::window::Event::subscription().map(|event| match event {
        //     iced::window::Event::Resized { width, height } => AppMessage::WindowResized(width, height),
        //     _ => AppMessage::NoOp,
        // })
    }

    ///
    /// Show a modal dialog over the `base_contents` of the window.
    ///
    fn modal<'a, AppMessage>(
        base_contents: impl Into<Element<'a, AppMessage>>,
        modal_contents: impl Into<Element<'a, AppMessage>>,
        on_press_event: AppMessage,
    ) -> Element<'a, AppMessage>
    where
        AppMessage: Clone + 'a,
    {
        stack![
            base_contents.into(),
            opaque(
                mouse_area(center(opaque(modal_contents)).style(|_theme| {
                    container::Style {
                        background: Some(
                            iced::Color {
                                a: 0.95,
                                ..iced::Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
                .on_press(on_press_event)
            )
        ]
            .into()
    }

}