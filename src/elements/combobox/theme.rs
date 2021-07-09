use seamonkey::*;

use interface::{ get_subtheme, get_float };
use elements::TextfieldTheme;

pub struct ElementTheme {
    pub textfield_theme: TextfieldTheme,
    pub padding: f32,
}

impl ElementTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            textfield_theme: TextfieldTheme::load(get_subtheme(&theme, "textfield")),
            padding: get_float(&theme, "padding", 0.0),
        }
    }
}
