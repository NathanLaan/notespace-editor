//!
//! Notespace-Editor.
//!
//! Toolbar for main app window.
//!

use std::ops::Deref;
use iced::{border, widget::{Container, container, button, horizontal_space, row, tooltip, text}, Element, Length, Theme};
use iced::widget::PickList;
use iced::widget::tooltip::Position;
use iced::widget::theme;
use super::app_message::AppMessage;
use super::app_state::AppState;
use super::app_const::{UI_CONTROL_SPACING, UI_CONTROL_PADDING, UI_TOOLBAR_BUTTON_SIZE, UI_TOOLTIP_PADDING};
use super::app_style::AppStyle;
use fa_iced as fa;
use iced::advanced::text::highlighter;
use iced::widget::container::Style;
use rust_i18n::t;
use crate::app_state;

pub struct AppToolbar;

impl AppToolbar {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn view(&self, app_state: &AppState) -> iced::Element<AppMessage> {
        let scale_picker = PickList::new(
            vec!(0.5,0.75,1.0,1.25,1.50,1.75,2.0,2.25,2.5,3.0,4.0),
            Some(app_state.scale_factor),
            AppMessage::UpdateScale,
        );
        let window_theme_picker = PickList::new(
            &Theme::ALL[..],
            Some(app_state.window_theme.clone()),
            AppMessage::UpdateWindowTheme,
        );
        let syntax_theme_picker = PickList::new(
            &iced::highlighter::Theme::ALL[..],
            Some(app_state.syntax_theme.clone()),
            AppMessage::UpdateSyntaxTheme,
        );

        let locale_list: Vec<String> = rust_i18n::available_locales!()
            .into_iter()
            .map(String::from)
            .collect();

        // Create an owned String
        let cur_locale = rust_i18n::locale().to_string();
        //let selected_locale = Some(cur_locale.clone());
        let selected_locale = Some(cur_locale);
        let lang_picker: PickList<String, Vec<String>, String, AppMessage> = PickList::new(
            locale_list,
            selected_locale,
            AppMessage::UpdateLanguage,
        );

        let row = row![
            create_button(fa::FA_ICON_NEW, "file_new", Some(AppMessage::NewFile)),
            create_button(fa::FA_ICON_OPEN, "file_open", Some(AppMessage::OpenFileFromDialog)),
            create_button(fa::FA_ICON_SAVE, "file_save",
                app_state.file_dirty.then_some(AppMessage::SaveFile)),
            horizontal_space(),
            scale_picker,
            window_theme_picker,
            syntax_theme_picker,
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
    //content: impl Into<Element<'a, Message>>,
    icon_key: &'a str,
    i18n_key: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let btn =
        button(container(fa::iced_text_icon(icon_key))
            .center_x(Length::Fill)
            .width(UI_TOOLBAR_BUTTON_SIZE))
            .on_press_maybe(on_press);
    let tooltip_text = t!(i18n_key);
    tooltip(
        btn,
        iced::widget::Text::new(tooltip_text),
        Position::FollowCursor,
    )
        .style(AppStyle::rounded_box)
        .into()
}
