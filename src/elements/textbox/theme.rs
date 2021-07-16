use seamonkey::*;

use elements::TextbufferTheme;

use interface::{ get_subtheme, get_float };

pub struct TextboxTheme {
    pub textbuffer_theme: TextbufferTheme,
    pub padding: f32,
}

impl TextboxTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            textbuffer_theme: TextbufferTheme::load(get_subtheme(&theme, "textbuffer")),
            padding: get_float(&theme, "padding", 0.0),
        }
    }
}
