//!
//! KeybindAction represents actions that a user can trigger using a keyboard shortcut (keybind).
//!

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeybindAction {
    CutText,
    CopyText,
    PasteText,
    NewFile,
    SaveFile,
    CloseFile,
    ShowSettings,
    QuitApplication,
}
