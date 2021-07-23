use seamonkey::*;

use themes::{ ItemTheme, TextfieldTheme };
use elements::ComboItem;

#[derive(Clone)]
pub struct NoteItem {
    text: SharedString,
    index: usize,
}

impl NoteItem {

    pub fn new(note: &Note) -> Self {
        return Self {
            text: format_shared!("line {}: {} - {}", note.position.line, note.kind.serialize(), note.message),
            index: note.position.index,
        }
    }
}

impl ComboItem for NoteItem {

    type Value = usize;

    fn display_name(&self) -> SharedString {
        return self.text.clone();
    }

    fn update_name(&self) -> SharedString {
        return self.text.clone();
    }

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme {
        return &theme.default_theme;
    }

    fn return_value(&self) -> Self::Value {
        return self.index;
    }
}
