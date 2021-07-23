use seamonkey::*;

use themes::{ ItemTheme, TextfieldTheme };
use elements::ComboItem;

#[derive(Clone)]
pub struct LanguageItem {
    name: SharedString,
    file_name: SharedString,
    recommended: bool,
}

impl LanguageItem {

    pub fn new(file_name: SharedString, recommendation: &SharedString) -> Self {
        let name = file_name.remove_str(&SharedString::from(".data"));
        let recommended = name == *recommendation;

        return Self {
            name: name,
            file_name: file_name,
            recommended: recommended,
        }
    }
}

impl ComboItem for LanguageItem {

    type Value = SharedString;

    fn display_name(&self) -> SharedString {
        return self.name.clone();
    }

    fn update_name(&self) -> SharedString {
        return self.name.clone();
    }

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme {
        match self.recommended {
            true => return &theme.special_theme,
            false => return &theme.default_theme,
        }
    }

    fn return_value(&self) -> Self::Value {
        return self.file_name.clone();
    }
}
