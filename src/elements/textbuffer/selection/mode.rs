#[derive(Copy, Clone, Debug)]
pub enum SelectionMode {
    Character,
    Word,
    Line,
}

impl SelectionMode {

    pub fn name(&self) -> &'static str {
        match self {
            SelectionMode::Character => "character",
            SelectionMode::Word => "word",
            SelectionMode::Line => "line",
        }
    }

    pub fn is_character(&self) -> bool {
        match self {
            SelectionMode::Character => return true,
            _other => return false,
        }
    }

    pub fn is_word(&self) -> bool {
        match self {
            SelectionMode::Word => return true,
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
