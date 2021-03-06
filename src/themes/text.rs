use seamonkey::*;

use sfml::graphics::{ Color, TextStyle };

use themes::*;

pub struct TextTheme {
    pub text_color: Color,
    pub text_style: TextStyle,
    pub border_width: f32,
    pub border_color: Color,
}

impl TextTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            text_color: get_color(&theme, "text_color", Color::BLACK),
            text_style: get_style(&theme, "text_style", TextStyle::REGULAR),
            border_width: get_float(&theme, "border_width", 0.0),
            border_color: get_color(&theme, "border_color", Color::WHITE),
        }
    }
}
