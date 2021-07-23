use seamonkey::SharedString;

#[derive(Clone)]
pub enum ComboSelection {
    TextBox,
    Item(usize, SharedString),
}

impl ComboSelection {

    pub fn is_textbox(&self) -> bool {
        match self {
            ComboSelection::TextBox => return true,
            _other => return false,
        }
    }

    pub fn index_matches(&self, selected: usize) -> bool {
        match self {
            ComboSelection::TextBox => return false,
            ComboSelection::Item(index, _original) => return *index == selected,
        }
    }
}
