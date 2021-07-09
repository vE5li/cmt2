use seamonkey::*;

use sfml::SfBox;
use sfml::window::Key;
use sfml::graphics::Font;

use input::*;
use input::Action;

const SMALLEST_FONT_SIZE: usize = 5;
const BIGGEST_FONT_SIZE: usize = 50;

const ANTIALIASING_MIN: usize = 0;
const ANTIALIASING_MAX: usize = 8;

pub struct InterfaceContext {
    pub font_size: usize,
    pub font: SfBox<Font>,
    pub bindings: Vec<(Binding, Action)>,
    pub selection_gap: usize,
    pub line_spacing: f32,
    pub character_spacing: f32,
    pub antialiasing_level: usize,
}

impl InterfaceContext {

    pub fn temp() -> Status<Self> {

        let font = Font::from_file("/home/.config/poet/fonts/monaco.ttf").expect("failed to load font");

        let bindings_file = format_shared!("/home/.config/poet/bindings.data");
        let bindings_data = confirm!(read_map(&bindings_file));

        let bindings_entry = confirm!(bindings_data.index(&keyword!("bindings"))).unwrap();
        let bindings_entry = unpack_map!(&bindings_entry);

        let mut bindings = Vec::new();

        for (key, value) in bindings_entry.iter() {
            let action = confirm!(Action::from_literal(&unpack_literal!(key)));

            let bindings_list = unpack_list!(value);
            for binding in bindings_list.iter() {

                let mut trigger = None;
                let mut included = Modifiers::new();
                let mut excluded = Modifiers::new();

                let binding_keys_list = unpack_list!(binding);
                for binding_key in binding_keys_list.iter() {

                    let key = confirm!(key_from_literal(&unpack_literal!(binding_key)));

                    if binding_key.is_keyword() {
                        if is_modifier_key(key) {
                            match key {
                                Key::LSHIFT => excluded.shift = true,
                                Key::LCONTROL => excluded.control = true,
                                Key::LALT => excluded.alt = true,
                                Key::LSYSTEM => excluded.system = true,
                                _other => panic!(),
                            }
                        } else {
                            return error!(string!("only modifiers can be excluded in bindings"));
                        }

                    } else {
                        if is_modifier_key(key) {
                            match key {
                                Key::LSHIFT => included.shift = true,
                                Key::LCONTROL => included.control = true,
                                Key::LALT => included.alt = true,
                                Key::LSYSTEM => included.system = true,
                                _other => panic!(),
                            }
                        } else {
                            trigger = Some(key);
                        }
                    }
                }

                let trigger = expect!(trigger, string!("keybinding must have a trigger"));
                let new_binding = Binding::new(trigger, included, excluded);
                match bindings.iter().position(|(binding, _): &(Binding, Action)| binding.length() <= new_binding.length()) {
                    Some(index) => bindings.insert(index, (new_binding, action)),
                    None => bindings.push((new_binding, action)),
                }
            }
        }

        return success!(Self {
            font_size: 14,
            font: font,
            bindings: bindings,
            selection_gap: 8,
            line_spacing: 1.4,
            character_spacing: 0.625,
            antialiasing_level: ANTIALIASING_MAX,
        });
    }

    pub fn get_matching_actions(&self, key_event: &KeyEvent) -> Vec<Action> {
        let mut actions = Vec::new();
        for (binding, action) in self.bindings.iter() {
            if binding.matches(&key_event.trigger, &key_event.modifiers) {
                if !actions.contains(action) {
                    actions.push(*action);
                }
            }
        }
        return actions;
    }

    pub fn zoom_in(&mut self) -> bool {
        if self.font_size < BIGGEST_FONT_SIZE {
            self.font_size += 1;
            return true;
        }
        return false;
    }

    pub fn zoom_out(&mut self) -> bool {
        if self.font_size > SMALLEST_FONT_SIZE {
            self.font_size -= 1;
            return true;
        }
        return false;
    }

    pub fn increase_antialiasing(&mut self) -> bool {
        if self.antialiasing_level < ANTIALIASING_MAX {
            self.antialiasing_level *= 2;
            return true;
        }
        return false;
    }

    pub fn decrease_antialiasing(&mut self) -> bool {
        if self.antialiasing_level > ANTIALIASING_MIN {
            self.antialiasing_level /= 2;
            return true;
        }
        return false;
    }
}
