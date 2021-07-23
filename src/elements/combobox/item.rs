use seamonkey::SharedString;

use themes::{ ItemTheme, TextfieldTheme };

pub trait ComboItem {

    type Value;

    fn display_name(&self) -> SharedString;

    fn update_name(&self) -> SharedString;

    fn display_theme<'t>(&self, theme: &'t ItemTheme) -> &'t TextfieldTheme;

    fn return_value(&self) -> Self::Value;
}
