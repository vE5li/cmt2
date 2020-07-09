pub enum SelectionMode {
    Character,
    Token,
    Line,
}

impl SelectionMode {

    pub fn name(&self) -> &'static str {
        match self {
            SelectionMode::Character => "character",
            SelectionMode::Token => "token",
            SelectionMode::Line => "line",
        }
    }

    pub fn is_character(&self) -> bool {
        match self {
            SelectionMode::Character => return true,
            _other => return false,
        }
    }

    pub fn is_token(&self) -> bool {
        match self {
            SelectionMode::Token => return true,
            _other => return false,
        }
    }

    pub fn is_line(&self) -> bool {
        match self {
            SelectionMode::Line => return true,
            _other => return false,
        }
    }
}
