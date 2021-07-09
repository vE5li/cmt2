use seamonkey::*;

use elements::TextfieldTheme;
use interface::{ get_subtheme, get_float };

pub struct PopupTheme {
    pub textfield_theme: TextfieldTheme,
    pub height: f32,
}

impl PopupTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            textfield_theme: TextfieldTheme::load(get_subtheme(&theme, "textfield")),
            height: get_float(&theme, "height", 1.5),
        }
    }
}

pub struct MessageTheme {
    pub info_theme: PopupTheme,
    pub warning_theme: PopupTheme,
    pub error_theme: PopupTheme,
}

impl MessageTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            info_theme: PopupTheme::load(get_subtheme(&theme, "info")),
            warning_theme: PopupTheme::load(get_subtheme(&theme, "warning")),
            error_theme: PopupTheme::load(get_subtheme(&theme, "error")),
        }
    }
}
