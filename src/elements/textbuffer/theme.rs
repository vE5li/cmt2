use seamonkey::*;

use sfml::system::Vector2f;

use elements::{ TextfieldTheme, TextTheme, FieldTheme, SelectionTheme };
use interface::{ get_subtheme, get_offset, get_float };

pub struct TextbufferTheme {
    pub background_theme: FieldTheme,
    pub selection_theme: SelectionTheme,
    pub new_selection_theme: SelectionTheme,
    pub selection_line_theme: FieldTheme,
    pub status_bar_theme: TextfieldTheme,
    pub line_number_theme: TextfieldTheme,
    pub text_theme: TextTheme,
    //pub comment_theme: TextTheme,
    //pub string_theme: TextTheme,
    // ...

    pub offset: Vector2f,
    pub line_number_width: f32,
    pub line_number_offset: f32,
    pub line_number_gap: f32,
}

impl TextbufferTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            background_theme: FieldTheme::load(get_subtheme(&theme, "background")),
            selection_theme: SelectionTheme::load(get_subtheme(&theme, "selection")),
            new_selection_theme: SelectionTheme::load(get_subtheme(&theme, "new_selection")),
            selection_line_theme: FieldTheme::load(get_subtheme(&theme, "selection_line")),
            status_bar_theme: TextfieldTheme::load(get_subtheme(&theme, "status_bar")),
            line_number_theme: TextfieldTheme::load(get_subtheme(&theme, "line_number")),
            text_theme: TextTheme::load(get_subtheme(&theme, "text")),
            //comment_theme: TextTheme::temp(),
            //string_theme: TextTheme::temp(),
            // ...

            offset: get_offset(&theme, "offset", Vector2f::new(1.0, 0.0)),
            line_number_width: get_float(&theme, "line_number_width", 4.0),
            line_number_offset: get_float(&theme, "line_number_offset", 1.0),
            line_number_gap: get_float(&theme, "line_number_gap", 0.0),
        }
    }
}
