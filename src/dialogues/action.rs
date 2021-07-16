use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use system::LanguageManager;
use dialogues::DialogueTheme;
use elements::ComboBox;
use interface::InterfaceContext;
use input::Action;

pub struct ActionDialogue {
    combobox: ComboBox,
}

impl ActionDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        let actions = vec![
            // no "comfirm", "action", "abort", "hidden_files"
            SharedString::from("quit"),
            SharedString::from("reload"),
            SharedString::from("append_lines"),
            SharedString::from("status_bar"),
            SharedString::from("line_numbers"),
            SharedString::from("selection_lines"),
            SharedString::from("highlighting"),
            SharedString::from("preserve_lines"),
            SharedString::from("start_at_symbol"),
            SharedString::from("unfocused_selections"),
            SharedString::from("character_mode"),
            SharedString::from("word_mode"),
            SharedString::from("line_mode"),
            SharedString::from("new_file"),
            SharedString::from("open_file"),
            SharedString::from("loaded_buffers"),
            SharedString::from("notes"),
            SharedString::from("save_file"),
            SharedString::from("set_language"),
            SharedString::from("set_theme"),
            SharedString::from("find_replace"),
            SharedString::from("start"),
            SharedString::from("end"),
            SharedString::from("extend_start"),
            SharedString::from("extend_end"),
            SharedString::from("add_selection"),
            SharedString::from("select_next"),
            SharedString::from("focus_next"),
            SharedString::from("remove"),
            SharedString::from("remove_section"),
            SharedString::from("delete"),
            SharedString::from("delete_line"),
            SharedString::from("zoom_in"),
            SharedString::from("zoom_out"),
            SharedString::from("increase_antialiasing"),
            SharedString::from("decrease_antialiasing"),
            SharedString::from("new_editor"),
            SharedString::from("close_window"),
            SharedString::from("page_up"),
            SharedString::from("page_down"),
            SharedString::from("extend_page_up"),
            SharedString::from("extend_page_down"),
            SharedString::from("duplicate_up"),
            SharedString::from("duplicate_down"),
            SharedString::from("insert"),
            SharedString::from("append"),
            SharedString::from("newline_up"),
            SharedString::from("newline_down"),
            SharedString::from("left"),
            SharedString::from("right"),
            SharedString::from("up"),
            SharedString::from("down"),
            SharedString::from("extend_left"),
            SharedString::from("extend_right"),
            SharedString::from("extend_up"),
            SharedString::from("extend_down"),
            SharedString::from("move_left"),
            SharedString::from("move_right"),
            SharedString::from("move_up"),
            SharedString::from("move_down"),
            SharedString::from("copy"),
            SharedString::from("paste"),
            SharedString::from("cut"),
            SharedString::from("rotate"),
        ];

        Self {
            combobox: ComboBox::new(language_manager, "action", 0, false, false, actions),
        }
    }

    pub fn get(&self) -> SharedString {
        return self.combobox.get();
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

        if let Action::Action = action {
            return (true, Some(false));
        }

        return self.combobox.handle_action(interface_context, language_manager, action);
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.combobox.add_character(language_manager, character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.combobox.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.combobox.render(framebuffer, interface_context, theme, true);
    }
}
