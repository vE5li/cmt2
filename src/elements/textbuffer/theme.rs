use seamonkey::*;

use sfml::system::Vector2f;

use elements::{ TextfieldTheme, TextTheme, FieldTheme, SelectionTheme };
use interface::{ get_subtheme, get_offset, get_float };

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

pub struct TextbufferTheme {
    pub background_theme: FieldTheme,
    pub selection_theme: SelectionTheme,
    pub new_selection_theme: SelectionTheme,
    pub selection_line_theme: FieldTheme,
    pub status_bar_theme: StatusBarTheme,
    pub line_number_theme: TextfieldTheme,
    pub highlighted_line_number_theme: TextfieldTheme,
    pub text_theme: TextTheme,
    pub comment_theme: TextTheme,
    pub operator_theme: TextTheme,
    pub keyword_theme: TextTheme,
    pub identifier_theme: TextTheme,
    pub type_identifier_theme: TextTheme,
    pub character_theme: TextTheme,
    pub string_theme: TextTheme,
    pub integer_theme: TextTheme,
    pub float_theme: TextTheme,
    pub invalid_theme: TextTheme,
    pub ignored_theme: TextTheme,
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
            status_bar_theme: StatusBarTheme::load(get_subtheme(&theme, "status_bar")),
            line_number_theme: TextfieldTheme::load(get_subtheme(&theme, "line_number")),
            highlighted_line_number_theme: TextfieldTheme::load(get_subtheme(&theme, "highlighted_line_number")),
            text_theme: TextTheme::load(get_subtheme(&theme, "text")),
            comment_theme: TextTheme::load(get_subtheme(&theme, "comment")),
            operator_theme: TextTheme::load(get_subtheme(&theme, "operator")),
            keyword_theme: TextTheme::load(get_subtheme(&theme, "keyword")),
            identifier_theme: TextTheme::load(get_subtheme(&theme, "identifier")),
            type_identifier_theme: TextTheme::load(get_subtheme(&theme, "type_identifier")),
            character_theme: TextTheme::load(get_subtheme(&theme, "character")),
            string_theme: TextTheme::load(get_subtheme(&theme, "string")),
            integer_theme: TextTheme::load(get_subtheme(&theme, "integer")),
            float_theme: TextTheme::load(get_subtheme(&theme, "float")),
            invalid_theme: TextTheme::load(get_subtheme(&theme, "invalid")),
            ignored_theme: TextTheme::load(get_subtheme(&theme, "ignored")),
            offset: get_offset(&theme, "offset", Vector2f::new(1.0, 0.0)),
            line_number_width: get_float(&theme, "line_number_width", 4.0),
            line_number_offset: get_float(&theme, "line_number_offset", 1.0),
            line_number_gap: get_float(&theme, "line_number_gap", 0.0),
        }
    }
}
