use seamonkey::*;

use sfml::graphics::Color;

use themes::*;

pub struct TextfieldTheme {
    pub field_theme: FieldTheme,
    pub text_theme: TextTheme,
    pub alignment: Alignment,
    pub text_offset: f32,
}

impl TextfieldTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            field_theme: FieldTheme::load(get_subtheme(&theme, "field")),
            text_theme: TextTheme::load(get_subtheme(&theme, "text")),
            alignment: get_alignment(&theme, "alignment", Alignment::Left),
            text_offset: get_float(&theme, "text_offset", 0.0),
        }
    }
}
