use sfml::graphics::{ Color, TextStyle };
use kami::*;

macro_rules! get_component {
    ($color:expr, $index:expr, $component:expr) => ({
        let component = confirm!($color.index(&integer!($index)));
        let component = expect!(component, Message, string!("missing {} component", $component));
        let component = unpack_integer!(component, Message, string!("invalid type for {} component", $component));
        ensure!(component >= 0 && component <= 255, Message, string!("invalid range for {} component", $component));
        component as u8
    })
}

macro_rules! get_color {
    ($theme:expr, $name:expr, $r:expr, $g:expr, $b:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {

            Some(color) => {
                let red = get_component!(color, 1, "red");
                let green = get_component!(color, 2, "green");
                let blue = get_component!(color, 3, "blue");
                Color::rgb(red, green, blue)
            },

            None => Color::rgb($r, $g, $b),
        }
    })
}

macro_rules! get_float {
    ($theme:expr, $name:expr, $default:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {
            Some(value) => unpack_float!(&value) as f32,
            None => $default,
        }
    })
}

macro_rules! get_integer {
    ($theme:expr, $name:expr, $default:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {
            Some(value) => unpack_integer!(&value) as usize,
            None => $default,
        }
    })
}

macro_rules! get_boolean {
    ($theme:expr, $name:expr, $default:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {
            Some(value) => unpack_boolean!(&value),
            None => $default,
        }
    })
}

macro_rules! get_string {
    ($theme:expr, $name:expr, $default:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {
            Some(value) => unpack_string!(&value),
            None => SharedString::from($default),
        }
    })
}

macro_rules! get_style {
    ($theme:expr, $name:expr, $default:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {

            Some(style) => {
                match unpack_identifier!(&style).printable().as_str() {
                    "regular" => TextStyle::REGULAR,
                    "bold" => TextStyle::BOLD,
                    "italic" => TextStyle::ITALIC,
                    invalid => return error!(Message, string!("invalid text style {}", invalid)),
                }
            },

            None => $default,
        }
    })
}

macro_rules! get_subtheme {
    ($theme:expr, $name:expr) => ({
        match confirm!($theme.index(&keyword!($name))) {
            Some(subtheme) => subtheme,
            None => map!(),
        }
    })
}

#[derive(Clone, Debug)]
pub struct PanelTheme {
    pub border: Color,
    pub background: Color,
    pub text: Color,
    pub comment: Color,
    pub string: Color,
    pub character: Color,
    pub integer: Color,
    pub float: Color,
    pub keyword: Color,
    pub operator: Color,
    pub identifier: Color,
    pub type_identifier: Color,
    pub error: Color,
    pub text_style: TextStyle,
    pub comment_style: TextStyle,
    pub string_style: TextStyle,
    pub character_style: TextStyle,
    pub integer_style: TextStyle,
    pub float_style: TextStyle,
    pub keyword_style: TextStyle,
    pub operator_style: TextStyle,
    pub identifier_style: TextStyle,
    pub type_identifier_style: TextStyle,
    pub error_style: TextStyle,
    pub gap: f32,
    pub radius: f32,
    pub left_offset: f32,
    pub top_offset: f32,
    pub right_offset: f32,
}

impl PanelTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            border: get_color!(theme, "border", 60, 60, 60),
            background: get_color!(theme, "background", 35, 35, 35),
            text: get_color!(theme, "text", 160, 160, 160),
            comment: get_color!(theme, "comment", 100, 100, 100),
            string: get_color!(theme, "string", 100, 150, 140),
            character: get_color!(theme, "character", 35, 155, 140),
            integer: get_color!(theme, "integer", 45, 110, 135),
            float: get_color!(theme, "float", 45, 110, 135),
            keyword: get_color!(theme, "keyword", 145, 100, 145),
            operator: get_color!(theme, "operator", 130, 130, 130),
            identifier: get_color!(theme, "identifier", 160, 160, 160),
            type_identifier: get_color!(theme, "type", 210, 100, 150),
            text_style: get_style!(theme, "text_style", TextStyle::REGULAR),
            comment_style: get_style!(theme, "comment_style", TextStyle::REGULAR),
            string_style: get_style!(theme, "string_style", TextStyle::REGULAR),
            character_style: get_style!(theme, "character_style", TextStyle::REGULAR),
            integer_style: get_style!(theme, "integer_style", TextStyle::REGULAR),
            float_style: get_style!(theme, "float_style", TextStyle::REGULAR),
            keyword_style: get_style!(theme, "keyword_style", TextStyle::REGULAR),
            operator_style: get_style!(theme, "operator_style", TextStyle::REGULAR),
            identifier_style: get_style!(theme, "identifier_style", TextStyle::REGULAR),
            type_identifier_style: get_style!(theme, "type_style", TextStyle::REGULAR),
            error_style: get_style!(theme, "error_style", TextStyle::BOLD),
            error: get_color!(theme, "error", 160, 60, 60),
            radius: get_float!(theme, "radius", 0.2),
            gap: get_float!(theme, "gap", 0.5),
            left_offset: get_float!(theme, "left_offset", 0.4),
            top_offset: get_float!(theme, "top_offset", 0.4),
            right_offset: get_float!(theme, "right_offset", 0.4),
        })
    }
}

#[derive(Clone, Debug)]
pub struct LineNumberTheme {
    pub background: Color,
    pub text: Color,
    pub text_style: TextStyle,
    pub width: f32,
    pub offset: f32,
    pub gap: f32,
    pub radius: f32,
    pub text_offset: f32,
}

impl LineNumberTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 45, 45, 45),
            text: get_color!(theme, "text", 100, 100, 100),
            text_style: get_style!(theme, "text_style", TextStyle::REGULAR),
            width: get_float!(theme, "width", 5.0),
            offset: get_float!(theme, "offset", 0.0),
            gap: get_float!(theme, "gap", 0.0),
            radius: get_float!(theme, "radius", 0.0),
            text_offset: get_float!(theme, "text_offset", 0.5),
        })
    }
}

#[derive(Clone, Debug)]
pub struct DialogueTheme {
    pub background: Color,
    pub focused: Color,
    pub text: Color,
    pub ghost: Color,
    pub focused_text: Color,
    pub focused_ghost: Color,
    pub text_style: TextStyle,
    pub ghost_style: TextStyle,
    pub height: f32,
}

impl DialogueTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 45, 45, 45),
            focused: get_color!(theme, "focused", 70, 70, 70),
            ghost: get_color!(theme, "ghost", 70, 70, 70),
            text: get_color!(theme, "text", 90, 90, 90),
            focused_text: get_color!(theme, "focused_text", 130, 130, 130),
            focused_ghost: get_color!(theme, "focused_ghost", 100, 100, 100),
            text_style: get_style!(theme, "text_style", TextStyle::REGULAR),
            ghost_style: get_style!(theme, "ghost_style", TextStyle::ITALIC),
            height: get_float!(theme, "height", 1.5),
        })
    }
}

#[derive(Clone, Debug)]
pub struct FocusBarTheme {
    pub background: Color,
    pub height: f32,
}

impl FocusBarTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 130, 80, 100),
            height: get_float!(theme, "height", 0.5),
        })
    }
}

#[derive(Clone, Debug)]
pub struct StatusBarTheme {
    pub background: Color,
    pub text: Color,
    pub text_style: TextStyle,
    pub height: f32,
    pub offset: f32,
}

impl StatusBarTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 130, 80, 100),
            text: get_color!(theme, "text", 100, 100, 100),
            text_style: get_style!(theme, "text_style", TextStyle::REGULAR),
            height: get_float!(theme, "height", 1.5),
            offset: get_float!(theme, "offset", 2.0),
        })
    }
}

#[derive(Clone, Debug)]
pub struct SelectionTheme {
    pub background: Color,
    pub text: Color,
    pub new_background: Color,
    pub new_text: Color,
    pub line: Color,
    pub text_style: TextStyle,
    pub radius: f32,
}

impl SelectionTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 115, 115, 115),
            text: get_color!(theme, "text", 35, 35, 35),
            new_background: get_color!(theme, "new_background", 145, 105, 130),
            new_text: get_color!(theme, "new_text", 35, 35, 35),
            line: get_color!(theme, "line", 50, 50, 50),
            text_style: get_style!(theme, "text_style", TextStyle::BOLD),
            radius: get_float!(theme, "radius", 0.05),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub panel: PanelTheme,
    pub line_number: LineNumberTheme,
    pub dialogue: DialogueTheme,
    pub status_bar: StatusBarTheme,
    pub focus_bar: FocusBarTheme,
    pub selection: SelectionTheme,
}

impl Theme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            panel: confirm!(PanelTheme::from(&get_subtheme!(theme, "panel"))),
            line_number: confirm!(LineNumberTheme::from(&get_subtheme!(theme, "line_number"))),
            dialogue: confirm!(DialogueTheme::from(&get_subtheme!(theme, "dialogue"))),
            status_bar: confirm!(StatusBarTheme::from(&get_subtheme!(theme, "status_bar"))),
            focus_bar: confirm!(FocusBarTheme::from(&get_subtheme!(theme, "focus_bar"))),
            selection: confirm!(SelectionTheme::from(&get_subtheme!(theme, "selection"))),
        })
    }
}
