use seamonkey::*;

use themes::*;

pub struct DialogueTheme {
    pub focused_textbox_theme: TextboxTheme,
    pub unfocused_textbox_theme: TextboxTheme,
    pub focused_item_theme: ItemTheme,
    pub unfocused_item_theme: ItemTheme,
    pub display_height: f32,
    pub height: f32,
}

impl DialogueTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            focused_textbox_theme: TextboxTheme::load(get_subtheme(&theme, "focused_textbox")),
            unfocused_textbox_theme: TextboxTheme::load(get_subtheme(&theme, "unfocused_textbox")),
            focused_item_theme: ItemTheme::load(get_subtheme(&theme, "focused_element")),
            unfocused_item_theme: ItemTheme::load(get_subtheme(&theme, "unfocused_element")),
            display_height: get_float(&theme, "display_height", 0.85),
            height: get_float(&theme, "height", 1.5),
        }
    }
}
