use kami::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Quit,
    ToggleAppendLines,
    ToggleStatusBar,
    ToggleLineNumbers,
    ToggleSelectionLines,
    MoveFocusLeft,
    MoveFocusRight,

    CharacterMode,
    TokenMode,
    LineMode,
    OpenFile,
    SetLanguage,
    Down,
    Up,
    Left,
    Right,
    Start,
    End,
    AddSelection,

    Abort,
    Confirm,
    Remove,
    Delete,
    Clear,

    ZoomIn,
    ZoomOut,
    ZoomInPanel,
    ZoomOutPanel,

    ExtendLeft,
    ExtendRight,
    ExtendUp,
    ExtendDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Copy,
    Paste,
    Cut,
}

impl Action {

    pub fn from_literal(literal: &VectorString) -> Status<Self> {
        match literal.printable().as_str() {
            "quit" => return success!(Action::Quit),
            "append_lines" => return success!(Action::ToggleAppendLines),
            "status_bar" => return success!(Action::ToggleStatusBar),
            "line_numbers" => return success!(Action::ToggleLineNumbers),
            "selection_lines" => return success!(Action::ToggleSelectionLines),
            "focus_left" => return success!(Action::MoveFocusLeft),
            "focus_right" => return success!(Action::MoveFocusRight),

            "character_mode" => return success!(Action::CharacterMode),
            "token_mode" => return success!(Action::TokenMode),
            "line_mode" => return success!(Action::LineMode),
            "open_file" => return success!(Action::OpenFile),
            "set_language" => return success!(Action::SetLanguage),
            "down" => return success!(Action::Down),
            "up" => return success!(Action::Up),
            "left" => return success!(Action::Left),
            "right" => return success!(Action::Right),
            "start" => return success!(Action::Start),
            "end" => return success!(Action::End),
            "add_selection" => return success!(Action::AddSelection),

            "abort" => return success!(Action::Abort),
            "confirm" => return success!(Action::Confirm),
            "remove" => return success!(Action::Remove),
            "delete" => return success!(Action::Delete),
            "clear" => return success!(Action::Clear),

            "zoom_in" => return success!(Action::ZoomIn),
            "zoom_out" => return success!(Action::ZoomOut),
            "zoom_in_panel" => return success!(Action::ZoomInPanel),
            "zoom_out_panel" => return success!(Action::ZoomOutPanel),

            "left" => return success!(Action::Left),
            "right" => return success!(Action::Right),
            "up" => return success!(Action::Up),
            "down" => return success!(Action::Down),
            "extend_left" => return success!(Action::ExtendLeft),
            "extend_right" => return success!(Action::ExtendRight),
            "extend_up" => return success!(Action::ExtendUp),
            "extend_down" => return success!(Action::ExtendDown),
            "move_left" => return success!(Action::MoveLeft),
            "move_right" => return success!(Action::MoveRight),
            "move_up" => return success!(Action::MoveUp),
            "move_down" => return success!(Action::MoveDown),
            "copy" => return success!(Action::Copy),
            "paste" => return success!(Action::Paste),
            "cut" => return success!(Action::Cut),
            invalid => return error!(Message, string!(str, "invalid action {}", invalid)),
        }
    }
}
