use seamonkey::*;

use sfml::graphics::Color;

use interface::{ Vector4f, get_vector, get_color, get_float };

pub struct FieldTheme {
    pub background_color: Color,
    pub corner_radius: Vector4f,
    pub border_width: f32,
    pub border_color: Color,
}

impl FieldTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            background_color: get_color(&theme, "background_color", Color::BLUE),
            corner_radius: get_vector(&theme, "corner_radius", Vector4f::with(0.0)),
            border_width: get_float(&theme, "border_width", 0.0),
            border_color: get_color(&theme, "border_color", Color::BLACK),
        }
    }
}
