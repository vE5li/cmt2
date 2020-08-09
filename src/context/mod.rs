#[macro_use]
mod theme;
mod action;

use kami::*;
pub use self::theme::Theme;
pub use self::action::Action;
use input::{ Binding, KeyEvent, Modifiers, key_from_literal, is_modifier_key };
use sfml::graphics::*;
use sfml::system::{ SfBox, Vector2f };
use sfml::window::Key;

pub struct Context {
    save_file: SharedString,
    pub line_numbers: bool,
    pub font_size: usize,
    pub line_spacing: f32,
    pub character_spacing: f32,
    pub selection_gap: usize,
    pub append_lines: bool,
    pub status_bar: bool,
    pub highlighting: bool,
    pub bindings: Vec<(Binding, Action)>,
    pub theme_name: SharedString,
    pub theme: Theme,
    pub font: SfBox<Font>,
    pub selection_lines: bool,
    pub preserve_lines: bool,
    pub unfocused_selections: bool,
    pub focus_bar: bool,
}

impl Context {

    pub fn new(save_file: SharedString, configuration_directory: &SharedString) -> Status<Self> {
        let mut configuration = map!();
        let mut bindings = Vec::new();

        let context_map = confirm!(read_map(&save_file));
        let context_map = get_subtheme!(context_map, "context");
        let line_numbers = get_boolean!(context_map, "line_numbers", true);
        let font_size = get_integer!(context_map, "font_size", 14);
        let line_spacing = get_float!(context_map, "line_spacing", 1.2);
        let character_spacing = get_float!(context_map, "spacing", 0.625);
        let selection_gap = get_integer!(context_map, "selection_gap", 8);
        let append_lines = get_boolean!(context_map, "append_lines", false);
        let status_bar = get_boolean!(context_map, "status_bar", true);
        let highlighting = get_boolean!(context_map, "highlighting", true);
        let selection_lines = get_boolean!(context_map, "selection_lines", true);
        let preserve_lines = get_boolean!(context_map, "preserve_lines", true);
        let unfocused_selections = get_boolean!(context_map, "show_selections", true);
        let focus_bar = get_boolean!(context_map, "focus_bar", true);
        let theme_name = get_string!(context_map, "theme", "default");

        let theme_file = format_shared!("/home/.poet/themes/{}.data", &theme_name);
        let theme_map = confirm!(read_map(&theme_file));
        let mut theme = confirm!(Theme::from(&theme_map));

        for mut entry in confirm!(get_directory_entries(configuration_directory)) {
            entry.insert_str(0, configuration_directory);
            let file_map = confirm!(read_map(&entry));
            configuration = confirm!(configuration.merge(&file_map));
        }

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
            line_numbers: line_numbers,
            font_size: font_size,
            line_spacing: line_spacing,
            character_spacing: character_spacing,
            selection_gap: selection_gap,
            append_lines: append_lines,
            status_bar: status_bar,
            highlighting: highlighting,
            bindings: bindings,
            theme_name: theme_name,
            theme: theme,
            font: font,
            selection_lines: selection_lines,
            preserve_lines: preserve_lines,
            unfocused_selections: unfocused_selections,
            focus_bar: focus_bar,
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

    pub fn toggle_highlighting(&mut self) {
        self.highlighting = !self.highlighting;
    }

    pub fn toggle_preserve_lines(&mut self) {
        self.preserve_lines = !self.preserve_lines;
    }

    pub fn toggle_unfocused_selections(&mut self) {
        self.unfocused_selections = !self.unfocused_selections;
    }

    pub fn toggle_focus_bar(&mut self) {
        self.focus_bar = !self.focus_bar;
    }

    pub fn safe(&self) {

    }
}
