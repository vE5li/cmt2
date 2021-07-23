use themes::*;

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
