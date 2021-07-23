use seamonkey::*;

use themes::*;

pub struct ItemTheme {
    pub default_theme: TextfieldTheme,
    pub special_theme: TextfieldTheme,
    pub padding: f32,
}

impl ItemTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            default_theme: TextfieldTheme::load(get_subtheme(&theme, "default")),
            special_theme: TextfieldTheme::load(get_subtheme(&theme, "special")),
            padding: get_float(&theme, "padding", 0.0),
        }
    }
}
