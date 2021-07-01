#[macro_use]
mod theme;
mod action;

use sfml::SfBox;
use sfml::graphics::*;
use sfml::system::{ Vector2f };
use sfml::window::Key;

use seamonkey::*;
use input::{ Binding, KeyEvent, Modifiers, key_from_literal, is_modifier_key };

pub use self::theme::Theme;
pub use self::action::Action;

const SMALLEST_FONT_SIZE: usize = 5;
const BIGGEST_FONT_SIZE: usize = 50;

const ANTIALIASING_MIN: usize = 0;
const ANTIALIASING_MAX: usize = 8;

pub struct Context {
    save_file: SharedString,
    pub line_numbers: bool,
    pub font_size: usize,
    pub tab_width: usize,
    pub scroll_size: usize,
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
    pub start_at_symbol: bool,
    pub antialiasing_level: usize,
}

impl Context {

    pub fn new(configuration_directory: &SharedString) -> Status<Self> {
        let mut configuration = map!();
        let mut bindings = Vec::new();
        let save_file = format_shared!("{}context.data", configuration_directory);

        let context_map = confirm!(read_map(&save_file));
        let context_map = get_subtheme!(context_map, "context");
        let line_numbers = get_boolean!(context_map, "line_numbers", true);
        let font_size = get_integer!(context_map, "font_size", 14);
        let tab_width = get_integer!(context_map, "tab_width", 4);
        let scroll_size = get_integer!(context_map, "scroll_size", 5);
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
        let start_at_symbol = get_boolean!(context_map, "start_at_symbol", true);
        let theme_name = get_string!(context_map, "theme", "default");
        let antialiasing_level = get_integer!(context_map, "antialiasing_level", 8);

        let theme_file = format_shared!("/home/.config/poet/themes/{}.data", &theme_name);
        let theme_map = confirm!(read_map(&theme_file));
        let mut theme = confirm!(Theme::from(&theme_map));

        let bindings_file = format_shared!("{}bindings.data", configuration_directory);
        let extentions_file = format_shared!("{}extentions.data", configuration_directory);

        let bindings_data = confirm!(read_map(&bindings_file));
        let extentions_data = confirm!(read_map(&extentions_file));

        configuration = confirm!(configuration.merge(&bindings_data));
        configuration = confirm!(configuration.merge(&extentions_data));

        let bindings_entry = confirm!(configuration.index(&keyword!("bindings"))).unwrap();
        let bindings_entry = unpack_map!(&bindings_entry);

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

        let font = Font::from_file("/home/.config/poet/fonts/monaco.ttf").expect("failed to load font");

        success!(Self {
            save_file: save_file,
            line_numbers: line_numbers,
            font_size: font_size,
            tab_width: tab_width,
            scroll_size: scroll_size,
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
            start_at_symbol: start_at_symbol,
            antialiasing_level: antialiasing_level,
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

    pub fn toggle_start_at_symbol(&mut self) {
        self.start_at_symbol = !self.start_at_symbol;
    }

    pub fn zoom_in(&mut self) {
        if self.font_size < BIGGEST_FONT_SIZE {
            self.font_size += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.font_size > SMALLEST_FONT_SIZE {
            self.font_size -= 1;
        }
    }

    pub fn increase_antialiasing(&mut self) {
        if self.antialiasing_level < ANTIALIASING_MAX {
            self.antialiasing_level *= 2;
        }
    }

    pub fn decrease_antialiasing(&mut self) {
        if self.antialiasing_level > ANTIALIASING_MIN {
            self.antialiasing_level /= 2;
        }
    }

    pub fn safe(&self) {

    }
}
