use seamonkey::*;

use sfml::graphics::{ Color, TextStyle };
use sfml::system::Vector2f;

use interface::Vector4f;
use elements::{ TextbufferTheme, MessageTheme, Alignment };
use dialogues::DialogueTheme;

pub fn get_subtheme(theme: &Option<Data>, name: &'static str) -> Option<Data> {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            return maybe_entry;
        }
    }
    return None
}

fn alignment_from_string(string: SharedString, name: &'static str, default_value: Alignment) -> Alignment {
    match string.serialize().as_str() {
        "left" => return Alignment::Left,
        "right" => return Alignment::Right,
        "center" => return Alignment::Center,
        other => println!("invalid \"{}\" alignment {}", name, other),
    }
    return default_value;
}

pub fn get_alignment(theme: &Option<Data>, name: &'static str, default_value: Alignment) -> Alignment {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                match entry {
                    Data::Identifier(literal) => return alignment_from_string(literal, name, default_value),
                    Data::String(literal) => return alignment_from_string(literal, name, default_value),
                    invalid => println!("invalid \"{}\" data {} for alignment", name, invalid.serialize()),
                }
            }
        }
    }
    return default_value;
}

fn style_from_string(string: SharedString, name: &'static str, default_value: TextStyle) -> TextStyle {
    match string.serialize().as_str() {
        "regular" => return TextStyle::REGULAR,
        "italic" => return TextStyle::ITALIC,
        "bold" => return TextStyle::BOLD,
        other => println!("invalid \"{}\" style {}", name, other),
    }
    return default_value;
}

pub fn get_style(theme: &Option<Data>, name: &'static str, default_value: TextStyle) -> TextStyle {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                match entry {
                    Data::Identifier(literal) => return style_from_string(literal, name, default_value),
                    Data::String(literal) => return style_from_string(literal, name, default_value),
                    invalid => println!("invalid \"{}\" data {} for style", name, invalid.serialize()),
                }
            }
        }
    }
    return default_value;
}

fn get_float_from_number(entry: &Data, name: &'static str, default_value: f32) -> f32 {
    match entry {
        Data::Float(float) => return *float as f32,
        Data::Integer(integer) => return *integer as f32,
        invalid => println!("\"{}\" expected float or integer; found {}", name, invalid.serialize()),
    }
    return default_value;
}

pub fn get_float(theme: &Option<Data>, name: &'static str, default_value: f32) -> f32 {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                return get_float_from_number(&entry, name, default_value);
            }
        }
    }
    return default_value;
}

pub fn get_u8_from_integer(data: &Data, name: &'static str, default_value: u8) -> u8 {

    if let Data::Integer(integer) = *data {
        if integer < 0 || integer > 255 {
            println!("color component \"{}\" must be between 0 and 255; found {}", name, integer);
            return default_value;
        }
        return integer as u8;
    }

    println!("\"{}\" expected integer; found {}", name, data.serialize());
    return default_value;
}

pub fn get_offset(theme: &Option<Data>, name: &'static str, default_value: Vector2f) -> Vector2f {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                if entry.is_list() {
                    let vector = extract_list!(entry);

                    if vector.len() == 2 {
                        let x = get_float_from_number(&vector[0], name, default_value.x);
                        let y = get_float_from_number(&vector[1], name, default_value.y);
                        return Vector2f::new(x, y);
                    }

                    println!("offset \"{}\" expected two items but got {}", name, vector.len());
                } else {
                    let value = get_float_from_number(&entry, name, 0.0);
                    return Vector2f::new(value, value);
                }
            }
        }
    }
    return default_value;
}

pub fn get_vector(theme: &Option<Data>, name: &'static str, default_value: Vector4f) -> Vector4f {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                if entry.is_list() {
                    let vector = extract_list!(entry);

                    if vector.len() == 4 {
                        let left = get_float_from_number(&vector[0], name, default_value.left);
                        let right = get_float_from_number(&vector[1], name, default_value.right);
                        let top = get_float_from_number(&vector[2], name, default_value.top);
                        let bottom = get_float_from_number(&vector[3], name, default_value.bottom);
                        return Vector4f::new(left, right, top, bottom);
                    }

                    println!("vector \"{}\" expected two items but got {}", name, vector.len());
                } else {
                    let value = get_float_from_number(&entry, name, 0.0);
                    return Vector4f::with(value);
                }
            }
        }
    }
    return default_value;
}

pub fn get_color(theme: &Option<Data>, name: &'static str, default_value: Color) -> Color {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                if entry.is_list() {
                    let vector = extract_list!(entry);
                    let length = vector.len();

                    if length >= 3 && length <= 4 {
                        let red = get_u8_from_integer(&vector[0], name, default_value.red());
                        let green = get_u8_from_integer(&vector[1], name, default_value.green());
                        let blue = get_u8_from_integer(&vector[2], name, default_value.blue());
                        let alpha = match length == 4 {
                            true => get_u8_from_integer(&vector[3], name, default_value.alpha()),
                            false => default_value.alpha(),
                        };

                        return Color::rgba(red, green, blue, alpha);
                    }

                    println!("color \"{}\" expected three or four items but got {}", name, vector.len());
                }
            }
        }
    }
    return default_value;
}

pub struct InterfaceTheme {
    pub name: SharedString,
    pub textbuffer_theme: TextbufferTheme,
    pub dialogue_theme: DialogueTheme,
    pub message_theme: MessageTheme,
}

impl InterfaceTheme {

    pub fn load(theme: Option<Data>) -> Self {
        return Self {
            name: SharedString::from("ree"),
            textbuffer_theme: TextbufferTheme::load(get_subtheme(&theme, "textbuffer")),
            dialogue_theme: DialogueTheme::load(get_subtheme(&theme, "dialogue")),
            message_theme: MessageTheme::load(get_subtheme(&theme, "message")),
        }
    }

    //pub fn temp() -> Self {
    //    return Self {
    //        name: SharedString::from("ree"),
    //        textbuffer_theme: TextbufferTheme::temp(),
    //        dialogue_theme: DialogueTheme::temp(),
    //        popup_theme: PopupTheme::temp(),
    //    }
    //}
}

/*
macro_rules! get_component {
    ($color:expr, $index:expr, $component:expr) => ({
        let component = confirm!($color.index(&integer!($index)));
        let component = expect!(component, string!("missing {} component", $component));
        let component = unpack_integer!(component, string!("invalid type for {} component", $component));
        ensure!(component >= 0 && component <= 255, string!("invalid range for {} component", $component));
        component as u8
    })
}

macro_rules! try_component {
    ($color:expr, $index:expr, $component:expr) => ({
        if let Some(component) = confirm!($color.index(&integer!($index))) {
            let component = unpack_integer!(component, string!("invalid type for {} component", $component));
            ensure!(component >= 0 && component <= 255, string!("invalid range for {} component", $component));
            component as u8
        } else {
            255
        }
    })
}

macro_rules! get_color {
    ($theme:expr, $name:expr, $r:expr, $g:expr, $b:expr, $a:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {

            Some(color) => {
                let red = get_component!(color, 1, "red");
                let green = get_component!(color, 2, "green");
                let blue = get_component!(color, 3, "blue");
                let alpha = try_component!(color, 4, "alpha");
                Color::rgba(red, green, blue, alpha)
            },

            None => Color::rgba($r, $g, $b, $a),
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
                    invalid => return error!(string!("invalid text style {}", invalid)),
                }
            },

            None => $default,
        }
    })
}

macro_rules! get_subtheme {
    ($theme:expr, $name:expr) => ({
        match confirm!($theme.index(&identifier!($name))) {
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
    pub left_offset: f32,
    pub top_offset: f32,
    pub right_offset: f32,
}

impl PanelTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            border: get_color!(theme, "border", 60, 60, 60, 255),
            background: get_color!(theme, "background", 35, 35, 35, 255),
            text: get_color!(theme, "text", 160, 160, 160, 255),
            comment: get_color!(theme, "comment", 100, 100, 100, 255),
            string: get_color!(theme, "string", 100, 150, 140, 255),
            character: get_color!(theme, "character", 35, 155, 140, 255),
            integer: get_color!(theme, "integer", 45, 110, 135, 255),
            float: get_color!(theme, "float", 45, 110, 135, 255),
            keyword: get_color!(theme, "keyword", 145, 100, 145, 255),
            operator: get_color!(theme, "operator", 130, 130, 130, 255),
            identifier: get_color!(theme, "identifier", 160, 160, 160, 255),
            type_identifier: get_color!(theme, "type", 210, 100, 150, 255),
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
            error: get_color!(theme, "error", 160, 60, 60, 255),
            //radius: get_float!(theme, "radius", 0.2),
            //gap: get_float!(theme, "gap", 0.5),
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
            background: get_color!(theme, "background", 45, 45, 45, 255),
            text: get_color!(theme, "text", 100, 100, 100, 255),
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
    pub corner_radius: f32,
    pub text_offset: f32,
}

impl DialogueTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            background: get_color!(theme, "background", 45, 45, 45, 255),
            focused: get_color!(theme, "focused", 70, 70, 70, 255),
            ghost: get_color!(theme, "ghost", 70, 70, 70, 255),
            text: get_color!(theme, "text", 90, 90, 90, 255),
            focused_text: get_color!(theme, "focused_text", 130, 130, 130, 255),
            focused_ghost: get_color!(theme, "focused_ghost", 100, 100, 100, 255),
            text_style: get_style!(theme, "text_style", TextStyle::REGULAR),
            ghost_style: get_style!(theme, "ghost_style", TextStyle::ITALIC),
            height: get_float!(theme, "height", 1.5),
            corner_radius: get_float!(theme, "corner_radius", 0.0),
            text_offset: get_float!(theme, "text_offset", 1.5),
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
            background: get_color!(theme, "background", 130, 80, 100, 255),
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
            background: get_color!(theme, "background", 130, 80, 100, 255),
            text: get_color!(theme, "text", 100, 100, 100, 255),
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
            background: get_color!(theme, "background", 115, 115, 115, 255),
            text: get_color!(theme, "text", 35, 35, 35, 255),
            new_background: get_color!(theme, "new_background", 145, 105, 130, 255),
            new_text: get_color!(theme, "new_text", 35, 35, 35, 255),
            line: get_color!(theme, "line", 50, 50, 50, 255),
            text_style: get_style!(theme, "text_style", TextStyle::BOLD),
            radius: get_float!(theme, "radius", 0.05),
        })
    }
}

#[derive(Clone, Debug)]
pub struct PopupTheme {
    pub info_background: Color,
    pub warning_background: Color,
    pub error_background: Color,
    pub info_text: Color,
    pub warning_text: Color,
    pub error_text: Color,
    pub info_style: TextStyle,
    pub warning_style: TextStyle,
    pub error_style: TextStyle,
    pub height: f32,
    pub corner_radius: f32,
    pub text_offset: f32,
}

impl PopupTheme {

    pub fn from(theme: &Data) -> Status<Self> {
        return success!(Self {
            info_background: get_color!(theme, "info_background", 115, 115, 115, 255),
            warning_background: get_color!(theme, "warning_background", 115, 115, 115, 255),
            error_background: get_color!(theme, "error_background", 115, 115, 115, 255),
            info_text: get_color!(theme, "info_text", 35, 35, 35, 255),
            warning_text: get_color!(theme, "warning_text", 35, 35, 35, 255),
            error_text: get_color!(theme, "error_text", 35, 35, 35, 255),
            info_style: get_style!(theme, "info_style", TextStyle::REGULAR),
            warning_style: get_style!(theme, "warning_style", TextStyle::REGULAR),
            error_style: get_style!(theme, "error_style", TextStyle::BOLD),
            height: get_float!(theme, "height", 1.5),
            corner_radius: get_float!(theme, "corner_radius", 0.0),
            text_offset: get_float!(theme, "text_offset", 1.5),
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
    pub popup: PopupTheme,
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
            popup: confirm!(PopupTheme::from(&get_subtheme!(theme, "popup"))),
        })
    }
}*/
