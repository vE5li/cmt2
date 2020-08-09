mod modifiers;
mod binding;
mod event;

use kami::*;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use sfml::window::Key;

pub use self::modifiers::Modifiers;
pub use self::binding::Binding;
pub use self::event::KeyEvent;

pub fn is_modifier_key(key: &Key) -> bool {
    match key {
        Key::LShift => return true,
        Key::LControl => return true,
        Key::LAlt => return true,
        Key::LSystem => return true,
        _other => return false,
    }
}

pub fn key_from_literal(literal: &SharedString) -> Status<Key> {
    match literal.printable().as_str() {
        "a" => return success!(Key::A),
        "b" => return success!(Key::B),
        "c" => return success!(Key::C),
        "d" => return success!(Key::D),
        "e" => return success!(Key::E),
        "f" => return success!(Key::F),
        "g" => return success!(Key::G),
        "h" => return success!(Key::H),
        "i" => return success!(Key::I),
        "j" => return success!(Key::J),
        "k" => return success!(Key::K),
        "l" => return success!(Key::L),
        "m" => return success!(Key::M),
        "n" => return success!(Key::N),
        "o" => return success!(Key::O),
        "p" => return success!(Key::P),
        "q" => return success!(Key::Q),
        "r" => return success!(Key::R),
        "s" => return success!(Key::S),
        "t" => return success!(Key::T),
        "u" => return success!(Key::U),
        "v" => return success!(Key::V),
        "w" => return success!(Key::W),
        "x" => return success!(Key::X),
        "y" => return success!(Key::Y),
        "z" => return success!(Key::Z),
        "0" => return success!(Key::Num0),
        "1" => return success!(Key::Num1),
        "2" => return success!(Key::Num2),
        "3" => return success!(Key::Num3),
        "4" => return success!(Key::Num4),
        "5" => return success!(Key::Num5),
        "6" => return success!(Key::Num6),
        "7" => return success!(Key::Num7),
        "8" => return success!(Key::Num8),
        "9" => return success!(Key::Num9),
        "escape" => return success!(Key::Escape),
        "control" => return success!(Key::LControl),
        "shift" => return success!(Key::LShift),
        "alt" => return success!(Key::LAlt),
        "system" => return success!(Key::LSystem),
        "menu" => return success!(Key::Menu),
        "left_bracket" => return success!(Key::LBracket),
        "right_bracket" => return success!(Key::RBracket),
        "semicolon" => return success!(Key::SemiColon),
        "comma" => return success!(Key::Comma),
        "period" => return success!(Key::Period),
        "quote" => return success!(Key::Quote),
        "slash" => return success!(Key::Slash),
        "backslash" => return success!(Key::BackSlash),
        "tilde" => return success!(Key::Tilde),
        "equal" => return success!(Key::Equal),
        "dash" => return success!(Key::Dash),
        "space" => return success!(Key::Space),
        "enter" => return success!(Key::Return),
        "backspace" => return success!(Key::BackSpace),
        "tab" => return success!(Key::Tab),
        "pageup" => return success!(Key::PageUp),
        "pagedown" => return success!(Key::PageDown),
        "end" => return success!(Key::End),
        "start" => return success!(Key::Home),
        "insert" => return success!(Key::Insert),
        "delete" => return success!(Key::Delete),
        "add" => return success!(Key::Add),
        "subtract" => return success!(Key::Subtract),
        "multiply" => return success!(Key::Multiply),
        "divide" => return success!(Key::Divide),
        "left" => return success!(Key::Left),
        "right" => return success!(Key::Right),
        "up" => return success!(Key::Up),
        "down" => return success!(Key::Down),
        "numpad_0" => return success!(Key::Numpad0),
        "numpad_1" => return success!(Key::Numpad1),
        "numpad_2" => return success!(Key::Numpad2),
        "numpad_3" => return success!(Key::Numpad3),
        "numpad_4" => return success!(Key::Numpad4),
        "numpad_5" => return success!(Key::Numpad5),
        "numpad_6" => return success!(Key::Numpad6),
        "numpad_7" => return success!(Key::Numpad7),
        "numpad_8" => return success!(Key::Numpad8),
        "numpad_9" => return success!(Key::Numpad9),
        "f1" => return success!(Key::F1),
        "f2" => return success!(Key::F2),
        "f3" => return success!(Key::F3),
        "f4" => return success!(Key::F4),
        "f5" => return success!(Key::F5),
        "f6" => return success!(Key::F6),
        "f7" => return success!(Key::F7),
        "f8" => return success!(Key::F8),
        "f9" => return success!(Key::F9),
        "f10" => return success!(Key::F10),
        "f11" => return success!(Key::F11),
        "f12" => return success!(Key::F12),
        "f13" => return success!(Key::F13),
        "f14" => return success!(Key::F14),
        "f15" => return success!(Key::F15),
        "pause" => return success!(Key::Pause),
        invalid => return error!(Message, string!("invalid key {}", invalid)),
    }
}

/*fn active_from_state(state: u8) -> bool {
    return state != 0;
}

fn toggle_active(active: bool, state: u8) -> bool {
    match state == 1 {
        true => return !active,
        false => return active,
    }
}

pub fn open_keyboard(device_name: &SharedString, sender: Sender<KeyEvent>) -> Status<()> {
    use std::fs::File;
    use std::io::Read;

    let mut codes: HashMap<u8, Key> = HashMap::new();
    let mut composites: Vec<(Binding, Key)> = Vec::new();
    let driver_file_path = format_shared!("/home/.poet/input/{}.data", device_name);
    let driver_map = confirm!(read_map(&driver_file_path));

    let codes_map = confirm!(driver_map.index(&keyword!("codes"))).unwrap();
    let codes_map = unpack_map!(&codes_map);

    for (key, value) in codes_map.iter() {
        let key = confirm!(Key::from_literal(&unpack_literal!(key)));
        let codes_list = unpack_list!(value);
        for code in codes_list.iter() {
            let code = unpack_integer!(code);
            ensure!(code >= 0  && code <= 255, Message, string!("code out of range"));
            codes.insert(code as u8, key.clone());
        }
    }

    let composites_map = confirm!(driver_map.index(&keyword!("composite"))).unwrap();
    let composites_map = unpack_map!(&composites_map);

    for (key, value) in composites_map.iter() {
        let key = confirm!(Key::from_literal(&unpack_literal!(key)));
        ensure!(!key.is_buffer() && !key.is_modifier(), Message, string!("only named keys or characters may be composed"));

        let bindings_list = unpack_list!(value);
        for binding in bindings_list.iter() {

            let mut trigger = None;
            let mut included = Vec::new();
            let mut excluded = Vec::new();

            let binding_keys_list = unpack_list!(binding);
            for binding_key in binding_keys_list.iter() {

                if binding_key.is_keyword() {
                    match confirm!(Key::from_literal(&unpack_keyword!(binding_key))) {

                        Key::Modifier(modifier_type) => {
                            ensure!(!excluded.contains(&modifier_type), Message, string!("duplicate excluded modifier"));
                            excluded.push(modifier_type);
                        },

                        _other => return error!(Message, string!("only modifiers can be excluded in bindings")),
                    }
                } else {
                    match confirm!(Key::from_literal(&unpack_literal!(binding_key))) {

                        Key::Modifier(modifier_type) => {
                            ensure!(!included.contains(&modifier_type), Message, string!("duplicate included modifier"));
                            included.push(modifier_type);
                        },

                        Key::Character(character) => {
                            ensure!(trigger.is_none(), Message, string!("trigger may only be set once"));
                            trigger = Some(Key::Character(character));
                        },

                        Key::Named(named) => {
                            ensure!(trigger.is_none(), Message, string!("trigger may only be set once"));
                            trigger = Some(Key::Named(named));
                        },

                        Key::Buffer(index) => {
                            ensure!(trigger.is_none(), Message, string!("trigger may only be set once"));
                            trigger = Some(Key::Buffer(index));
                        },
                    }
                }
            }

            let trigger = expect!(trigger, Message, string!("keybinding must have a trigger"));
            let new_binding = Binding::new(included, excluded, trigger);
            match composites.iter().position(|(binding, _)| binding.length() <= new_binding.length()) {
                Some(index) => composites.insert(index, (new_binding, key)),
                None => composites.push((new_binding, key)),
            }
        }
    }

    let device_file_path = format!("/dev/input/by-id/{}", device_name);
    let mut modifiers = Modifiers::new();

    let mut width: usize = 24;
    let mut buffer: Vec<u8> = Vec::with_capacity(width);
    unsafe { buffer.set_len(width) };

    loop {
        if let Ok(mut device_file) = File::open(&device_file_path) {
            while let Ok(_) = device_file.read_exact(&mut buffer) {
                if buffer[16] == 1 {
                    if let Some(key) = codes.get(&buffer[18]) {

                        if let Key::Modifier(modifier_type) = key {
                            match &modifier_type {
                                ModifierKey::SuperKey => modifiers.super_key = active_from_state(buffer[20]),
                                ModifierKey::Shift => modifiers.shift = active_from_state(buffer[20]),
                                ModifierKey::Control => modifiers.control = active_from_state(buffer[20]),
                                ModifierKey::Alt => modifiers.alt = active_from_state(buffer[20]),
                                ModifierKey::AltGraph => modifiers.alt_graph = active_from_state(buffer[20]),
                                ModifierKey::CapsLock => modifiers.caps_lock = toggle_active(modifiers.caps_lock, buffer[20]),
                                ModifierKey::Function => modifiers.function = active_from_state(buffer[20]),
                                ModifierKey::NumLock => modifiers.num_lock = toggle_active(modifiers.num_lock, buffer[20]),
                                ModifierKey::ScrollLock => modifiers.scroll_lock = toggle_active(modifiers.scroll_lock, buffer[20]),
                            }
                            continue;
                        }

                        if active_from_state(buffer[20]) {
                            let composed_key = match composites.iter().find(|(binding, _composed_key)| binding.matches(&key, &modifiers)) {
                                Some((_binding, composed_key)) => Some(*composed_key),
                                None => None,
                            };
                            sender.send(KeyEvent::new(*key, modifiers, composed_key));
                        }
                    }

                    //println!("[ code: {} ]", buffer[18]);
                }
            }
        } else {
            //thread::sleep(Duration::new(2, 0));
        }
    }
}*/
