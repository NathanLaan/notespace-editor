//!
//! Notespace-Editor.
//!
//! Toolbar for main app window.
//!

use crate::app_message::AppMessage;
use crate::app_state::AppState;
use crate::keyboard::keybind_action::KeybindAction;
use crate::ui_const::{
    UI_CONTROL_PADDING, UI_CONTROL_SPACING, UI_TOOLBAR_BUTTON_SIZE, UI_TOOLBAR_ICON_SIZE,
};
use crate::ui_style::AppStyle;
use crate::ui_util::create_toolbar_button_small;
use fa_iced as fa;
use iced::widget::PickList;
use iced::widget::tooltip::Position;
use iced::{
    Element, Length, Theme,
    widget::{Container, button, container, horizontal_space, row, tooltip},
};
use rust_i18n::t;

pub struct AppToolbar;

impl AppToolbar {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self, app_state: &AppState) -> Element<AppMessage> {
        let scale_factor_picker = PickList::new(
            vec![0.5, 0.75, 1.0, 1.25, 1.50, 1.75, 2.0, 2.25, 2.5, 3.0, 4.0],
            Some(app_state.scale_factor),
            AppMessage::UpdateScale,
        );
        let scale_factor_tooltip_text = t!("scale_factor");
        let scale_factor_tooltip = tooltip(
            scale_factor_picker,
            iced::widget::Text::new(scale_factor_tooltip_text),
            Position::FollowCursor,
        )
        .style(AppStyle::style_tooltip);

        let window_theme_picker = PickList::new(
            &Theme::ALL[..],
            Some(app_state.window_theme.clone()),
            AppMessage::UpdateWindowTheme,
        );
        let window_theme_tooltip_text = t!("window_theme");
        let window_theme_tooltip = tooltip(
            window_theme_picker,
            iced::widget::Text::new(window_theme_tooltip_text),
            Position::FollowCursor,
        )
        .style(AppStyle::style_tooltip);

        let syntax_theme_picker = PickList::new(
            &iced::highlighter::Theme::ALL[..],
            Some(app_state.syntax_theme.clone()),
            AppMessage::UpdateSyntaxTheme,
        );
        let syntax_theme_tooltip_text = t!("syntax_theme");
        let syntax_theme_tooltip = tooltip(
            syntax_theme_picker,
            iced::widget::Text::new(syntax_theme_tooltip_text),
            Position::FollowCursor,
        )
        .style(AppStyle::style_tooltip);

        let locale_list: Vec<String> = rust_i18n::available_locales!()
            .into_iter()
            .map(String::from)
            .collect();

        // Create an owned String
        let cur_locale = rust_i18n::locale().to_string();
        let selected_locale = Some(cur_locale);
        let locale_picker = PickList::new(locale_list, selected_locale, AppMessage::UpdateLanguage);
        let locale_tooltip_text = t!("language");
        let locale_tooltip = tooltip(
            locale_picker,
            iced::widget::Text::new(locale_tooltip_text),
            Position::FollowCursor,
        )
        .style(AppStyle::style_tooltip);

        let row = row![
            create_toolbar_button_small(
                fa::FA_ICON_NEW,
                "file_new",
                Some(AppMessage::NewFile),
                true,
                KeybindAction::NewFile,
                &app_state,
            ),
            create_toolbar_button_small(
                fa::FA_ICON_OPEN,
                "file_open",
                Some(AppMessage::OpenFileFromDialog),
                true,
                KeybindAction::OpenFile,
                &app_state,
            ),
            create_toolbar_button_small(
                fa::FA_ICON_SAVE,
                "file_save",
                app_state.file_dirty.then_some(AppMessage::SaveFile),
                true,
                KeybindAction::SaveFile,
                &app_state,
            ),
            create_toolbar_button_small(
                fa::FA_ICON_GEAR,
                "app_configuration",
                Some(AppMessage::OpenAppConfigurationModal),
                true,
                KeybindAction::ShowSettings,
                &app_state,
            ),
            create_toolbar_button_small(
                fa::FA_ICON_INFO,
                "app_information",
                Some(AppMessage::OpenAppConfigurationModal),
                true,
                KeybindAction::ShowSettings,
                &app_state,
            ),
            horizontal_space(),
            scale_factor_tooltip,
            window_theme_tooltip,
            syntax_theme_tooltip,
            locale_tooltip,
        ]
        .spacing(UI_CONTROL_SPACING)
        .padding(UI_CONTROL_PADDING);

        // Wrap the Row in a styled Container to set background color
        Container::new(row)
            //
            // TODO: Need to figure out iced 0.13 style.
            //
            //.style(iced::theme::Theme::Custom(Box::new(ColoredBackground(Color::from_rgb(0.5, 0.5, 0.5)))))
            .style(AppStyle::style_toolbar)
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
    let btn = button(
        container(fa::iced_text_icon_solid(icon_key, UI_TOOLBAR_ICON_SIZE))
            .center_x(Length::Fill)
            .width(UI_TOOLBAR_BUTTON_SIZE),
    )
    .on_press_maybe(on_press);
    let locale_tooltip_text = t!(i18n_key);
    tooltip(
        btn,
        iced::widget::Text::new(locale_tooltip_text),
        Position::FollowCursor,
    )
    .style(AppStyle::style_tooltip)
    .into()
}
