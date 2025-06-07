//!
//! Smartcard-Iced-App
//!
//! Keybind Manager. Map `KeybindAction` to `(Modifiers, Code)` combinations.
//!
use crate::keyboard::keybind_action::KeybindAction;
use iced::keyboard::Modifiers;
use iced::keyboard::key::Code;
use iced::keyboard::key::Key;
use iced::keyboard::key::Named;
use lazy_static::lazy_static;
use libutil::dbg_println;
use std::collections::HashMap;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KeybindManager {
    pub bindings: HashMap<(iced::keyboard::Modifiers, iced::keyboard::key::Code), KeybindAction>,
}

impl Default for KeybindManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(
            (Modifiers::CTRL, iced::keyboard::key::Code::KeyQ),
            KeybindAction::QuitApplication,
        );
        bindings.insert(
            (Modifiers::CTRL, iced::keyboard::key::Code::KeyK),
            KeybindAction::ShowSettings,
        );
        bindings.insert(
            (Modifiers::CTRL, iced::keyboard::key::Code::KeyN),
            KeybindAction::NewFile,
        );
        bindings.insert(
            (Modifiers::CTRL, iced::keyboard::key::Code::KeyS),
            KeybindAction::SaveFile,
        );
        //
        // TODO: remaining keybinds
        //
        Self { bindings }
    }
}

impl KeybindManager {
    ///
    /// Get the AppAction corresponding to the key and modifiers, if one exists.
    ///
    pub fn get_app_action(&self, key: &Key, modifiers: Modifiers) -> Option<KeybindAction> {
        // Convert the Key to a Code, if possible
        match KEY_MAP.get(&key) {
            Some(code) => {
                let key_tuple = &(modifiers, *code);
                // Check for a matching binding
                self.bindings.get(key_tuple).copied()
            }
            None => {
                dbg_println!("KeybindManager - Unknown key {:?}", key);
                None
            }
        }
    }

    ///
    /// Get `Some<(Modifiers, Code)' for the specified `keybind_action` or `None` if
    /// no associated keybinds are found.
    ///
    pub fn get_key_for_action(&self, keybind_action: KeybindAction) -> Option<(Modifiers, Code)> {
        self.bindings.iter().find_map(|(key, val)| {
            if val == &keybind_action {
                Some(*key)
            } else {
                None
            }
        })
    }

    ///
    /// Get a user-displayable (but not localized) String representation of the
    /// keybind associated with the specified `keybind_action`. Returns an empty
    /// string if no keybind is found.
    ///
    pub fn get_display_text_for_action(&self, keybind_action: KeybindAction) -> String {
        match self.get_key_for_action(keybind_action) {
            Some(value) => KeybindManager::get_display_text_for_keybind(value.0, value.1),
            None => String::new(),
        }
    }

    ///
    /// Internal function called by `get_display_text_for_action`.
    ///
    fn get_display_text_for_keybind(mods: Modifiers, code: Code) -> String {
        let mut parts = Vec::new();

        if mods.contains(Modifiers::CTRL) {
            parts.push("Ctrl");
        }
        if mods.contains(Modifiers::SHIFT) {
            parts.push("Shift");
        }
        if mods.contains(Modifiers::ALT) {
            parts.push("Alt");
        }
        if mods.contains(Modifiers::LOGO) {
            parts.push("Super");
        }

        // Convert Code to a string
        let code_str = match code {
            Code::KeyA => "A",
            Code::KeyB => "B",
            Code::KeyC => "C",
            Code::KeyD => "D",
            Code::KeyE => "E",
            Code::KeyF => "F",
            Code::KeyG => "G",
            Code::KeyH => "H",
            Code::KeyI => "I",
            Code::KeyJ => "J",
            Code::KeyK => "K",
            Code::KeyL => "L",
            Code::KeyM => "M",
            Code::KeyN => "N",
            Code::KeyO => "O",
            Code::KeyP => "P",
            Code::KeyQ => "Q",
            Code::KeyR => "R",
            Code::KeyS => "S",
            Code::KeyT => "T",
            Code::KeyU => "U",
            Code::KeyV => "V",
            Code::KeyW => "W",
            Code::KeyX => "X",
            Code::KeyY => "Y",
            Code::KeyZ => "Z",
            Code::F1 => "F1",
            Code::F2 => "F2",
            Code::F3 => "F3",
            Code::F4 => "F4",
            Code::F5 => "F5",
            Code::F6 => "F6",
            Code::F7 => "F7",
            Code::F8 => "F8",
            Code::F9 => "F9",
            Code::F10 => "F10",
            Code::F11 => "F11",
            Code::F12 => "F12",
            Code::Enter => "Enter",
            Code::Space => "Space",
            Code::Escape => "Esc",
            Code::Tab => "Tab",
            Code::Backspace => "Backspace",
            Code::Delete => "Del",
            Code::ArrowUp => "↑",
            Code::ArrowDown => "↓",
            Code::ArrowLeft => "←",
            Code::ArrowRight => "→",
            other => return format!("{:?}", other), // Fallback for unsupported keys
        };

        parts.push(code_str);
        parts.join("+")
    }

    ///
    /// Map `iced::keyboard::key::Code` to `iced::keyboard::key::Key`.
    ///
    /// Returns `Some<iced::keyboard::key::Key>` if a mapping is found, or `None`.
    ///
    /// The `iced::keyboard::key::Code` struct is a platform-agnostic representation
    /// of the physical key location on the keyboard, while `iced_core::keyboard::Key`
    /// is a high-level logical representation of the key.
    ///
    fn _map_code_to_key(code: iced::keyboard::key::Code) -> Option<Key> {
        match code {
            Code::KeyA => Some(Key::Character("a".into())),
            Code::KeyB => Some(Key::Character("b".into())),
            Code::KeyC => Some(Key::Character("c".into())),
            Code::KeyD => Some(Key::Character("d".into())),
            Code::KeyE => Some(Key::Character("e".into())),
            Code::KeyF => Some(Key::Character("f".into())),
            Code::KeyG => Some(Key::Character("g".into())),
            Code::KeyH => Some(Key::Character("h".into())),
            Code::KeyI => Some(Key::Character("i".into())),
            Code::KeyJ => Some(Key::Character("j".into())),
            Code::KeyK => Some(Key::Character("k".into())),
            Code::KeyL => Some(Key::Character("l".into())),
            Code::KeyM => Some(Key::Character("m".into())),
            Code::KeyN => Some(Key::Character("n".into())),
            Code::KeyO => Some(Key::Character("o".into())),
            Code::KeyP => Some(Key::Character("p".into())),
            Code::KeyQ => Some(Key::Character("q".into())),
            Code::KeyR => Some(Key::Character("r".into())),
            Code::KeyS => Some(Key::Character("s".into())),
            Code::KeyT => Some(Key::Character("t".into())),
            Code::KeyU => Some(Key::Character("u".into())),
            Code::KeyV => Some(Key::Character("v".into())),
            Code::KeyW => Some(Key::Character("w".into())),
            Code::KeyX => Some(Key::Character("x".into())),
            Code::KeyY => Some(Key::Character("y".into())),
            Code::KeyZ => Some(Key::Character("z".into())),

            Code::Digit1 => Some(Key::Character("1".into())),
            Code::Digit2 => Some(Key::Character("2".into())),
            Code::Digit3 => Some(Key::Character("3".into())),
            Code::Digit4 => Some(Key::Character("4".into())),
            Code::Digit5 => Some(Key::Character("5".into())),
            Code::Digit6 => Some(Key::Character("6".into())),
            Code::Digit7 => Some(Key::Character("7".into())),
            Code::Digit8 => Some(Key::Character("8".into())),
            Code::Digit9 => Some(Key::Character("9".into())),
            Code::Digit0 => Some(Key::Character("0".into())),

            Code::Minus => Some(Key::Character("-".into())),
            Code::Equal => Some(Key::Character("=".into())),
            Code::BracketLeft => Some(Key::Character("[".into())),
            Code::BracketRight => Some(Key::Character("]".into())),
            Code::Backslash => Some(Key::Character("\\".into())),
            Code::Semicolon => Some(Key::Character(";".into())),
            Code::Quote => Some(Key::Character("'".into())),
            Code::Comma => Some(Key::Character(",".into())),
            Code::Period => Some(Key::Character(".".into())),
            Code::Slash => Some(Key::Character("/".into())),
            Code::Space => Some(Key::Character(" ".into())),
            Code::Tab => Some(Key::Named(Named::Tab)),
            Code::Enter => Some(Key::Named(Named::Enter)),
            Code::Backspace => Some(Key::Named(Named::Backspace)),
            Code::Escape => Some(Key::Named(Named::Escape)),
            Code::Delete => Some(Key::Named(Named::Delete)),
            Code::Insert => Some(Key::Named(Named::Insert)),
            Code::Home => Some(Key::Named(Named::Home)),
            Code::End => Some(Key::Named(Named::End)),
            Code::PageUp => Some(Key::Named(Named::PageUp)),
            Code::PageDown => Some(Key::Named(Named::PageDown)),

            Code::ArrowLeft => Some(Key::Named(Named::ArrowLeft)),
            Code::ArrowRight => Some(Key::Named(Named::ArrowRight)),
            Code::ArrowUp => Some(Key::Named(Named::ArrowUp)),
            Code::ArrowDown => Some(Key::Named(Named::ArrowDown)),

            Code::F1 => Some(Key::Named(Named::F1)),
            Code::F2 => Some(Key::Named(Named::F2)),
            Code::F3 => Some(Key::Named(Named::F3)),
            Code::F4 => Some(Key::Named(Named::F4)),
            Code::F5 => Some(Key::Named(Named::F5)),
            Code::F6 => Some(Key::Named(Named::F6)),
            Code::F7 => Some(Key::Named(Named::F7)),
            Code::F8 => Some(Key::Named(Named::F8)),
            Code::F9 => Some(Key::Named(Named::F9)),
            Code::F10 => Some(Key::Named(Named::F10)),
            Code::F11 => Some(Key::Named(Named::F11)),
            Code::F12 => Some(Key::Named(Named::F12)),

            Code::Numpad0 => Some(Key::Character("0".into())),
            Code::Numpad1 => Some(Key::Character("1".into())),
            Code::Numpad2 => Some(Key::Character("2".into())),
            Code::Numpad3 => Some(Key::Character("3".into())),
            Code::Numpad4 => Some(Key::Character("4".into())),
            Code::Numpad5 => Some(Key::Character("5".into())),
            Code::Numpad6 => Some(Key::Character("6".into())),
            Code::Numpad7 => Some(Key::Character("7".into())),
            Code::Numpad8 => Some(Key::Character("8".into())),
            Code::Numpad9 => Some(Key::Character("9".into())),
            Code::NumpadDecimal => Some(Key::Character(".".into())),
            Code::NumpadDivide => Some(Key::Character("/".into())),
            Code::NumpadMultiply => Some(Key::Character("*".into())),
            Code::NumpadSubtract => Some(Key::Character("-".into())),
            Code::NumpadAdd => Some(Key::Character("+".into())),
            Code::NumpadEnter => Some(Key::Named(Named::Enter)), // Numpad Enter might be distinct in some cases

            Code::ShiftLeft => Some(Key::Named(Named::Shift)),
            Code::ShiftRight => Some(Key::Named(Named::Shift)),
            Code::ControlLeft => Some(Key::Named(Named::Control)),
            Code::ControlRight => Some(Key::Named(Named::Control)),
            Code::AltLeft => Some(Key::Named(Named::Alt)),
            Code::AltRight => Some(Key::Named(Named::Alt)),
            //Code::MetaLeft => Some(Key::Super),
            //Code::MetaRight => Some(Key::Super),
            //Code::MetaLeft => Some(Key::Logo),

            // Add mappings for other relevant `Code` variants as needed
            _ => None, // Return None for unhandled `Code` values
        }
    }

    //
    // TODO: load and save the key bindings...
    //
}

lazy_static! {
    ///
    /// Static singleton pattern.
    ///
    pub static ref KEY_MAP: HashMap<iced::keyboard::key::Key, iced::keyboard::key::Code> = {
        let mut m = HashMap::new();
        m.insert(Key::Character("a".into()), Code::KeyA);
        m.insert(Key::Character("b".into()), Code::KeyB);
        m.insert(Key::Character("c".into()), Code::KeyC);
        m.insert(Key::Character("d".into()), Code::KeyD);
        m.insert(Key::Character("e".into()), Code::KeyE);
        m.insert(Key::Character("f".into()), Code::KeyF);
        m.insert(Key::Character("g".into()), Code::KeyG);
        m.insert(Key::Character("h".into()), Code::KeyH);
        m.insert(Key::Character("i".into()), Code::KeyI);
        m.insert(Key::Character("j".into()), Code::KeyJ);
        m.insert(Key::Character("k".into()), Code::KeyK);
        m.insert(Key::Character("l".into()), Code::KeyL);
        m.insert(Key::Character("m".into()), Code::KeyM);
        m.insert(Key::Character("n".into()), Code::KeyN);
        m.insert(Key::Character("o".into()), Code::KeyO);
        m.insert(Key::Character("p".into()), Code::KeyP);
        m.insert(Key::Character("q".into()), Code::KeyQ);
        m.insert(Key::Character("r".into()), Code::KeyR);
        m.insert(Key::Character("s".into()), Code::KeyS);
        m.insert(Key::Character("t".into()), Code::KeyT);
        m.insert(Key::Character("u".into()), Code::KeyU);
        m.insert(Key::Character("v".into()), Code::KeyV);
        m.insert(Key::Character("w".into()), Code::KeyW);
        m.insert(Key::Character("x".into()), Code::KeyX);
        m.insert(Key::Character("y".into()), Code::KeyY);
        m.insert(Key::Character("z".into()), Code::KeyZ);
        m
    };

}

//
// Example declaration:
//
//static KEY_A: Key = Key::Character(SmolStr::new_inline("a"));
