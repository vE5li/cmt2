use seamonkey::*;

use elements::{ TextboxTheme, ElementTheme };
use interface::{ get_subtheme, get_float };

pub struct DialogueTheme {
    pub focused_textbox_theme: TextboxTheme,
    pub unfocused_textbox_theme: TextboxTheme,
    pub focused_element_theme: ElementTheme,
    pub unfocused_element_theme: ElementTheme,
    pub display_height: f32,
    pub height: f32,
}

impl DialogueTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            focused_textbox_theme: TextboxTheme::load(get_subtheme(&theme, "focused_textbox")),
            unfocused_textbox_theme: TextboxTheme::load(get_subtheme(&theme, "unfocused_textbox")),
            focused_element_theme: ElementTheme::load(get_subtheme(&theme, "focused_element")),
            unfocused_element_theme: ElementTheme::load(get_subtheme(&theme, "unfocused_element")),
            display_height: get_float(&theme, "display_height", 0.85),
            height: get_float(&theme, "height", 1.5),
        }
    }
}
