//!
//! Notespace-Editor
//!
//! Iced application messages.
//!

use crate::app_io::AppIOError;
use iced::widget::text_editor;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum AppMessage {
    TextEdited(text_editor::Action),
    OpenFileFromDialog,
    FileOpened(Result<(PathBuf, Arc<String>), AppIOError>),
    NewFile,
    SaveFile,
    FileSaved(Result<PathBuf, AppIOError>),
    UpdateLanguage(String),
    UpdateWindowTheme(iced::Theme),
    UpdateSyntaxTheme(iced::highlighter::Theme),
    UpdateScale(f64),
    //KeyPressed(iced::keyboard::Key, iced::keyboard::Modifiers),
    //KeyPressedEvent(iced::keyboard::Key),
    EventOccurred(iced::Event),
    //WindowResized(f32, f32),
    SaveAppConfiguration,
    TabPressed,
    FocusChanged(Option<iced::advanced::widget::Id>),
    OpenAppConfigurationModal,
    CloseAppConfigurationModal,
}
