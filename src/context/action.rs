use seamonkey::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum Action {
    Quit,
    ToggleAppendLines,
    ToggleStatusBar,
    ToggleLineNumbers,
    ToggleSelectionLines,
    ToggleHighlighting,
    TogglePreserveLines,
    ToggleUnfocusedSelections,
    ToggleFocusBar,

    CharacterMode,
    TokenMode,
    LineMode,
    OpenFile,
    SaveFile,
    SetLanguage,
    FindReplace,
    Down,
    Up,
    Left,
    Right,
    Start,
    End,
    ExtendStart,
    ExtendEnd,
    AddSelection,
    SelectNext,
    FocusNext,

    Action,

    Abort,
    Confirm,
    Remove,
    Delete,
    DeleteLine,

    ZoomIn,
    ZoomOut,

    NewEditor,
    CloseWindow,

    PageUp,
    PageDown,
    ExtendPageUp,
    ExtendPageDown,

    DuplicateUp,
    DuplicateDown,

    Insert,
    Append,

    NewlineUp,
    NewlineDown,

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
    Rotate,
}

impl Action {

    pub fn from_literal(literal: &SharedString) -> Status<Self> {
        match literal.printable().as_str() {
            "quit" => return success!(Action::Quit),
            "append_lines" => return success!(Action::ToggleAppendLines),
            "status_bar" => return success!(Action::ToggleStatusBar),
            "line_numbers" => return success!(Action::ToggleLineNumbers),
            "selection_lines" => return success!(Action::ToggleSelectionLines),
            "highlighting" => return success!(Action::ToggleHighlighting),
            "unfocused_selections" => return success!(Action::ToggleUnfocusedSelections),
            "focus_bar" => return success!(Action::ToggleFocusBar),

            "character_mode" => return success!(Action::CharacterMode),
            "token_mode" => return success!(Action::TokenMode),
            "line_mode" => return success!(Action::LineMode),
            "open_file" => return success!(Action::OpenFile),
            "save_file" => return success!(Action::SaveFile),
            "set_language" => return success!(Action::SetLanguage),
            "find_replace" => return success!(Action::FindReplace),
            "start" => return success!(Action::Start),
            "end" => return success!(Action::End),
            "extend_start" => return success!(Action::ExtendStart),
            "extend_end" => return success!(Action::ExtendEnd),
            "add_selection" => return success!(Action::AddSelection),
            "select_next" => return success!(Action::SelectNext),
            "focus_next" => return success!(Action::FocusNext),

            "action" => return success!(Action::Action),

            "abort" => return success!(Action::Abort),
            "confirm" => return success!(Action::Confirm),
            "remove" => return success!(Action::Remove),
            "delete" => return success!(Action::Delete),
            "delete_line" => return success!(Action::DeleteLine),

            "zoom_in" => return success!(Action::ZoomIn),
            "zoom_out" => return success!(Action::ZoomOut),

            "new_editor" => return success!(Action::NewEditor),
            "close_window" => return success!(Action::CloseWindow),

            "page_up" => return success!(Action::PageUp),
            "page_down" => return success!(Action::PageDown),
            "extend_page_up" => return success!(Action::ExtendPageUp),
            "extend_page_down" => return success!(Action::ExtendPageDown),

            "duplicate_up" => return success!(Action::DuplicateUp),
            "duplicate_down" => return success!(Action::DuplicateDown),

            "insert" => return success!(Action::Insert),
            "append" => return success!(Action::Append),

            "newline_up" => return success!(Action::NewlineUp),
            "newline_down" => return success!(Action::NewlineDown),

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
            "rotate" => return success!(Action::Rotate),

            invalid => return error!(string!("invalid action {}", invalid)),
        }
    }
}
