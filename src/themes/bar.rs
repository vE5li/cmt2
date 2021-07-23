use themes::*;

pub struct StatusBarTheme {
    pub textfield_theme: TextfieldTheme,
    pub height: f32,
}

impl StatusBarTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            textfield_theme: TextfieldTheme::load(get_subtheme(&theme, "textfield")),
            height: get_float(&theme, "height", 1.5),
        }
    }
}
