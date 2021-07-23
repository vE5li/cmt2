use seamonkey::*;

use input::Action;
use themes::{ ItemTheme, TextfieldTheme };
use elements::ComboItem;

#[derive(Clone)]
pub struct ActionItem {
    action: Action,
    name: SharedString,
}

impl ActionItem {

    pub fn new(action: Action, name: &'static str) -> Self {
        return Self {
            action: action,
            name: SharedString::from(name),
        }
    }
}

impl ComboItem for ActionItem {

    type Value = Action;

    fn display_name(&self) -> SharedString {
        return self.name.clone();
    }

    fn update_name(&self) -> SharedString {
        return self.name.clone();
    }

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme {
        match self.action.is_global() {
            true => return &theme.special_theme,
            false => return &theme.default_theme,
        }
    }

    fn return_value(&self) -> Self::Value {
        return self.action;
    }
}
