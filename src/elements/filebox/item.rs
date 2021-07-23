use seamonkey::*;

use themes::{ ItemTheme, TextfieldTheme };
use elements::ComboItem;

#[derive(Clone)]
pub struct FileItem {
    file_name: SharedString,
    full_path: SharedString,
    directory: bool,
}

impl FileItem {

    pub fn new(file_name: SharedString, path: &SharedString) -> Self {
        let directory = *file_name.chars().last().unwrap() == Character::from_char('/');
        let full_path = format_shared!("{}{}", path, file_name);

        return Self {
            file_name: file_name,
            full_path: full_path,
            directory: directory,
        }
    }
}

impl ComboItem for FileItem {

    type Value = SharedString;

    fn display_name(&self) -> SharedString {
        return self.file_name.clone();
    }

    fn update_name(&self) -> SharedString {
        return self.full_path.clone();
    }

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme {
        match self.directory {
            true => return &theme.special_theme,
            false => return &theme.default_theme,
        }
    }

    fn return_value(&self) -> Self::Value {
        return self.full_path.clone();
    }
}
