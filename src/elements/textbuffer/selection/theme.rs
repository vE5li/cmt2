use seamonkey::*;

use elements::TextfieldTheme;
use interface::get_subtheme;

pub struct SelectionTheme {
    pub single_selection_theme: TextfieldTheme,
    pub first_selection_theme: TextfieldTheme,
    pub last_selection_theme: TextfieldTheme,
    pub center_selection_theme: TextfieldTheme,
}

impl SelectionTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            single_selection_theme: TextfieldTheme::load(get_subtheme(&theme, "single_selection")),
            first_selection_theme: TextfieldTheme::load(get_subtheme(&theme, "first_selection")),
            last_selection_theme: TextfieldTheme::load(get_subtheme(&theme, "last_selection")),
            center_selection_theme: TextfieldTheme::load(get_subtheme(&theme, "center_selection")),
        }
    }
}
