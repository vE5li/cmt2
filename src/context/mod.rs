mod theme;
mod action;

use kami::*;
pub use self::theme::Theme;
pub use self::action::Action;
use input::{ Binding, KeyEvent, Modifiers, key_from_literal, is_modifier_key };
use sfml::graphics::*;
use sfml::system::{ SfBox, Vector2f };
use sfml::window::Key;

macro_rules! unpack_component {
    ($color:expr, $index:expr, $component:expr) => ({
        let component = confirm!($color.index(&integer!($index)));
        let component = expect!(component, Message, string!("missing {} component", $component));
        let component = unpack_integer!(component, Message, string!("invalid type for {} component", $component));
        ensure!(component >= 0 && component <= 255, Message, string!("invalid range for {} component", $component));
        component as u8
    })
}

macro_rules! fetch_color {
    ($colors:expr, $name:expr, $theme:expr, $field:ident) => ({
        if let Some(color) = confirm!($colors.index(&identifier!($name))) {
            let red = unpack_component!(color, 1, "red");
            let green = unpack_component!(color, 2, "green");
            let blue = unpack_component!(color, 3, "blue");
            $theme.$field = Color::rgb(red, green, blue);
        }
    })
}

pub struct Context {
    save_file: VectorString,
    pub line_numbers: bool,
    pub font_size: usize,
    pub line_spacing: f32,
    pub character_spacing: f32,
    pub width: usize,
    pub height: usize,
    pub selection_gap: usize,
    pub append_lines: bool,
    pub status_bar: bool,
    pub highlighting: bool,
    pub bindings: Vec<(Binding, Action)>,
    pub theme: Theme,
    pub font: SfBox<Font>,
    pub selection_lines: bool,
}

impl Context {

    pub fn new(save_file: VectorString, configuration_directory: &VectorString) -> Status<Self> {
        let mut configuration = map!();
        let mut bindings = Vec::new();
        let mut theme = Theme::new();

        for mut entry in confirm!(get_directory_entries(configuration_directory)) {
            entry.insert_str(0, configuration_directory);
            let file_map = confirm!(read_map(&entry));
            configuration = confirm!(configuration.merge(&file_map));
        }

        //if let Some(colors) = confirm!(configuration.index(&keyword!("colors"))) {
        //    fetch_color!(colors, "border", theme, border_color);
        //    fetch_color!(colors, "panel", theme, panel_color);
        //    fetch_color!(colors, "text", theme, text_color);
        //    fetch_color!(colors, "overlay", theme, overlay_color);
        //    fetch_color!(colors, "comment", theme, comment_color);
        //    fetch_color!(colors, "string", theme, string_color);
        //    fetch_color!(colors, "character", theme, character_color);
        //    fetch_color!(colors, "integer", theme, integer_color);
        //    fetch_color!(colors, "float", theme, float_color);
        //    fetch_color!(colors, "keyword", theme, keyword_color);
        //    fetch_color!(colors, "operator", theme, operator_color);
        //    fetch_color!(colors, "identifier", theme, identifier_color);
        //    fetch_color!(colors, "type_identifier", theme, type_identifier_color);
        //    fetch_color!(colors, "error", theme, error_color);
        //    fetch_color!(colors, "selection", theme, selection_color);
        //    fetch_color!(colors, "new_selection", theme, new_selection_color);
        //    fetch_color!(colors, "dialogue", theme, dialogue_color);
        //    fetch_color!(colors, "dialogue_focused", theme, dialogue_focused_color);
        //    // fetch element radius
        //    // fetch panel radius
        //    // fetch panel gap
        //}

        let bindings_map = confirm!(configuration.index(&keyword!("bindings"))).unwrap();
        let bindings_map = unpack_map!(&bindings_map);

        for (key, value) in bindings_map.iter() {
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
                        if is_modifier_key(&key) {
                            match key {
                                Key::LShift => excluded.shift = true,
                                Key::LControl => excluded.control = true,
                                Key::LAlt => excluded.alt = true,
                                Key::LSystem => excluded.system = true,
                                _other => panic!(),
                            }
                        } else {
                            return error!(Message, string!("only modifiers can be excluded in bindings"));
                        }

                    } else {
                        if is_modifier_key(&key) {
                            match key {
                                Key::LShift => included.shift = true,
                                Key::LControl => included.control = true,
                                Key::LAlt => included.alt = true,
                                Key::LSystem => included.system = true,
                                _other => panic!(),
                            }
                        } else {
                            trigger = Some(key);
                        }
                    }
                }

                let trigger = expect!(trigger, Message, string!("keybinding must have a trigger"));
                let new_binding = Binding::new(trigger, included, excluded);
                match bindings.iter().position(|(binding, _): &(Binding, Action)| binding.length() <= new_binding.length()) {
                    Some(index) => bindings.insert(index, (new_binding, action)),
                    None => bindings.push((new_binding, action)),
                }
            }
        }

        let font = Font::from_file("/home/.poet/fonts/monaco.ttf").expect("failed to load font");

        success!(Self {
            save_file: save_file,
            line_numbers: true,
            font_size: 14,
            line_spacing: 1.2,
            character_spacing: 0.625,
            width: 150,
            height: 50,
            selection_gap: 8,
            append_lines: false,
            status_bar: true,
            highlighting: true,
            bindings: bindings,
            theme: theme,
            font: font,
            selection_lines: true,
        })
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

    pub fn toggle_line_numbers(&mut self) {
        self.line_numbers = !self.line_numbers;
    }

    pub fn toggle_append_lines(&mut self) {
        self.append_lines = !self.append_lines;
    }

    pub fn toggle_status_bar(&mut self) {
        self.status_bar = !self.status_bar;
    }

    pub fn toggle_selection_lines(&mut self) {
        self.selection_lines = !self.selection_lines;
    }

    pub fn safe(&self) {

    }
}
