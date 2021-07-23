use seamonkey::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum Action {
    Quit,
    Reload,
    ToggleAppendLines,
    ToggleStatusBar,
    ToggleLineNumbers,
    ToggleSelectionLines,
    ToggleHighlighting,
    TogglePreserveLines,
    ToggleUnfocusedSelections,
    ToggleHiddenFiles,
    ToggleStartAtSymbol,
    ToggleRelativeLineNumbers,
    CharacterMode,
    WordMode,
    LineMode,
    NewFile,
    Open,
    Filebuffers,
    Notes,
    SaveFile,
    Language,
    Theme,
    Replace,
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
    RemoveSection,
    Delete,
    DeleteLine,
    ZoomIn,
    ZoomOut,
    IncreaseAntialiasing,
    DecreaseAntialiasing,
    NewWindow,
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
    Undo,
    Redo,
}

impl Action {

    pub fn from_literal(literal: &SharedString) -> Status<Self> {
        match literal.printable().as_str() {
            "quit" => return success!(Action::Quit),
            "reload" => return success!(Action::Reload),
            "append_lines" => return success!(Action::ToggleAppendLines),
            "status_bar" => return success!(Action::ToggleStatusBar),
            "line_numbers" => return success!(Action::ToggleLineNumbers),
            "selection_lines" => return success!(Action::ToggleSelectionLines),
            "highlighting" => return success!(Action::ToggleHighlighting),
            "unfocused_selections" => return success!(Action::ToggleUnfocusedSelections),
            "hidden_files" => return success!(Action::ToggleHiddenFiles),
            "start_at_symbol" => return success!(Action::ToggleStartAtSymbol),
            "relative_line_numbers" => return success!(Action::ToggleRelativeLineNumbers),
            "preserve_lines" => return success!(Action::TogglePreserveLines),
            "character_mode" => return success!(Action::CharacterMode),
            "word_mode" => return success!(Action::WordMode),
            "line_mode" => return success!(Action::LineMode),
            "new_file" => return success!(Action::NewFile),
            "open_file" => return success!(Action::Open),
            "loaded_buffers" => return success!(Action::Filebuffers),
            "notes" => return success!(Action::Notes),
            "save_file" => return success!(Action::SaveFile),
            "set_language" => return success!(Action::Language),
            "set_theme" => return success!(Action::Theme),
            "find_replace" => return success!(Action::Replace),
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
            "remove_section" => return success!(Action::RemoveSection),
            "delete" => return success!(Action::Delete),
            "delete_line" => return success!(Action::DeleteLine),
            "zoom_in" => return success!(Action::ZoomIn),
            "zoom_out" => return success!(Action::ZoomOut),
            "increase_antialiasing" => return success!(Action::IncreaseAntialiasing),
            "decrease_antialiasing" => return success!(Action::DecreaseAntialiasing),
            "new_editor" => return success!(Action::NewWindow),
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
            "undo" => return success!(Action::Undo),
            "redo" => return success!(Action::Redo),
            invalid => return error!(string!("invalid action {}", invalid)),
        }
    }

    pub fn is_global(&self) -> bool {
        match self {
            Action::Quit => return true,
            Action::Reload => return true,
            Action::ToggleAppendLines => return true,
            Action::ToggleStatusBar => return true,
            Action::ToggleLineNumbers => return true,
            Action::ToggleSelectionLines => return true,
            Action::ToggleHighlighting => return true,
            Action::ToggleUnfocusedSelections => return true,
            Action::ToggleStartAtSymbol => return true,
            Action::TogglePreserveLines => return true,
            Action::ToggleRelativeLineNumbers => return true,
            Action::ZoomIn => return true,
            Action::ZoomOut => return true,
            Action::IncreaseAntialiasing => return true,
            Action::DecreaseAntialiasing => return true,
            Action::NewWindow => return true,
            Action::CloseWindow => return true,
            _unhandled => return false,
        }
    }
}
