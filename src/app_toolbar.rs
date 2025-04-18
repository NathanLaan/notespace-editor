//!
//! Notespace-Editor.
//!
//! Toolbar for main app window.
//!

use iced::{widget::{Container, container, button, horizontal_space, row}, Element, Length};
use iced::widget::PickList;
use super::app_message::AppMessage;
use super::app_state::AppState;
use super::app_const::{UI_CONTROL_SPACING, UI_CONTROL_PADDING, UI_TOOLBAR_BUTTON_SIZE};
use super::app_style::ColoredBackground;
use fa_iced as fa;

pub struct AppToolbar;

impl AppToolbar {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn view(&self, app_state: &AppState) -> iced::Element<AppMessage> {
        let locale_list = rust_i18n::available_locales!();
        //
        // TODO: Get the current locale!
        //
        //let locale_string: String = String::from(rust_i18n::locale());
        //let cur: &str = rust_i18n::locale();
        // let locale_value = rust_i18n::locale();
        // let cur = locale_value.as_ref();
        let lang_picker: PickList<&str, Vec<&str>, &str, AppMessage> = PickList::new(
            locale_list,
            Some(""),
            AppMessage::UpdateLanguage,
        );

        let row = row![
            create_button(iced_text_icon_new(), "New", Some(AppMessage::NewFile)),
            create_button(iced_text_icon_open(), "Open", Some(AppMessage::OpenFileFromDialog)),
            create_button(iced_text_icon_save(), "Save", Some(AppMessage::SaveFile)),
            horizontal_space(),
            lang_picker,
        ]
            .spacing(UI_CONTROL_SPACING)
            .padding(UI_CONTROL_PADDING);

        // Wrap the Row in a styled Container to set background color
        Container::new(row)
            //
            // TODO: Need to figure out iced 0.13 style.
            //
            //.style(iced::theme::Theme::Custom(Box::new(ColoredBackground(Color::from_rgb(0.5, 0.5, 0.5)))))
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }
}

///
/// Create a button with the specified element and message handler.
///
fn create_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let btn = button(container(content).center_x(Length::Fill).width(UI_TOOLBAR_BUTTON_SIZE));
    if let Some(on_press) = on_press {
        btn.on_press(on_press).into()
    } else {
        btn.into()
    }
}

fn iced_text_icon_new<'a, Message>() -> Element<'a, Message> {
    fa::iced_text_icon(fa::FA_ICON_NEW)
}
fn iced_text_icon_open<'a, Message>() -> Element<'a, Message> {
    fa::iced_text_icon(fa::FA_ICON_OPEN)
}
fn iced_text_icon_save<'a, Message>() -> Element<'a, Message> {
    fa::iced_text_icon(fa::FA_ICON_SAVE)
}
