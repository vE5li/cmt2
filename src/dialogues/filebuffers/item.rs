use seamonkey::*;

use themes::{ ItemTheme, TextfieldTheme };
use elements::ComboItem;

#[derive(Clone)]
pub struct BufferItem {
    file_name: SharedString,
}

impl BufferItem {

    pub fn new(file_name: &str) -> Self {
        return Self {
            file_name: SharedString::from(file_name),
        }
    }
}

impl ComboItem for BufferItem {

    type Value = SharedString;

    fn display_name(&self) -> SharedString {
        return self.file_name.clone();
    }

    fn update_name(&self) -> SharedString {
        return self.file_name.clone();
    }

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme {
        return &theme.default_theme;
    }

    fn return_value(&self) -> Self::Value {
        return self.file_name.clone();
    }
}
