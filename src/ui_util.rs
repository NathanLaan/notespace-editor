use crate::app_const::UI_TOOLBAR_ICON_SIZE;
use crate::app_state::AppState;
use crate::keyboard::keybind_action::KeybindAction;
use crate::ui_const::UI_TOOLBAR_BUTTON_HEIGHT_SMALL;
use crate::ui_style::AppStyle;
use fa_iced as fa;
use iced::widget::tooltip::Position;
use iced::widget::{TextInput, button, checkbox, container, horizontal_space, row, text, tooltip};
use iced::{Element, Length};
use rust_i18n::t;

///
/// Create a button with the specified icon, tooltip text, and message handler.
///
pub(crate) fn create_toolbar_button_small<'a, Message: Clone + 'a>(
    icon_key: &'a str,
    i18n_key: &'a str,
    on_press: Option<Message>,
    enabled: bool,
    keybind_action: KeybindAction,
    app_state: &AppState,
) -> Element<'a, Message> {
    let tooltip_shortcut = app_state
        .keybind_manager
        .get_display_text_for_action(keybind_action);
    let btn = button(
        container(fa::iced_text_icon_solid(icon_key, UI_TOOLBAR_ICON_SIZE))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(UI_TOOLBAR_BUTTON_HEIGHT_SMALL)
            .height(UI_TOOLBAR_BUTTON_HEIGHT_SMALL),
    )
    .style(AppStyle::button_style_primary)
    .on_press_maybe(if enabled { on_press } else { None });
    let locale_tooltip_text =
        create_tooltip_text_with_shortcut(i18n_key, tooltip_shortcut.as_str());
    tooltip(
        btn,
        iced::widget::Text::new(locale_tooltip_text),
        Position::FollowCursor,
    )
    .style(AppStyle::style_tooltip)
    .into()
}

fn create_tooltip_text_with_shortcut(i18n_key: &str, shortcut: &str) -> String {
    if shortcut.is_empty() {
        t!(i18n_key).to_string()
    } else {
        format!("{} [{}]", t!(i18n_key), shortcut)
    }
}
