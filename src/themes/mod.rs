mod alignment;
mod bar;
mod dialogue;
mod field;
mod interface;
mod item;
mod message;
mod popup;
mod selection;
mod text;
mod textbox;
mod textbuffer;
mod textfield;

pub use self::alignment::Alignment;
pub use self::bar::StatusBarTheme;
pub use self::dialogue::DialogueTheme;
pub use self::field::FieldTheme;
pub use self::interface::InterfaceTheme;
pub use self::item::ItemTheme;
pub use self::message::MessageTheme;
pub use self::popup::PopupTheme;
pub use self::selection::SelectionTheme;
pub use self::text::TextTheme;
pub use self::textbox::TextboxTheme;
pub use self::textbuffer::TextbufferTheme;
pub use self::textfield::TextfieldTheme;

use seamonkey::*;

use sfml::graphics::{ Color, TextStyle };
use sfml::system::Vector2f;

use interface::Vector4f;

fn get_subtheme(theme: &Option<Data>, name: &'static str) -> Option<Data> {
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

fn get_alignment(theme: &Option<Data>, name: &'static str, default_value: Alignment) -> Alignment {
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

fn get_style(theme: &Option<Data>, name: &'static str, default_value: TextStyle) -> TextStyle {
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

fn get_float(theme: &Option<Data>, name: &'static str, default_value: f32) -> f32 {
    if let Some(theme) = theme {
        if let Status::Success(maybe_entry) = theme.index(&identifier!(name)) {
            if let Some(entry) = maybe_entry {
                return get_float_from_number(&entry, name, default_value);
            }
        }
    }
    return default_value;
}

fn get_u8_from_integer(data: &Data, name: &'static str, default_value: u8) -> u8 {

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

fn get_offset(theme: &Option<Data>, name: &'static str, default_value: Vector2f) -> Vector2f {
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

fn get_vector(theme: &Option<Data>, name: &'static str, default_value: Vector4f) -> Vector4f {
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

fn get_color(theme: &Option<Data>, name: &'static str, default_value: Color) -> Color {
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
                } else if entry.is_integer() {
                    let default_brightness = (default_value.red() + default_value.green() + default_value.blue()) / 3;
                    let brightness = get_u8_from_integer(&entry, name, default_brightness);
                    return Color::rgba(brightness, brightness, brightness, 255);
                }
            }
        }
    }
    return default_value;
}
