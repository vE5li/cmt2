use seamonkey::*;
use context::{ Context, Action };
use sfml::graphics::*;
use sfml::system::Vector2f;
use super::super::ComboBox;

pub struct ActionDialogue {
    pub action_box: ComboBox,
}

impl ActionDialogue {

    pub fn new() -> Self {
        let actions = vec![
            // no "comfirm", "action", "abort"
            SharedString::from("quit"),
            SharedString::from("append_lines"),
            SharedString::from("status_bar"),
            SharedString::from("line_numbers"),
            SharedString::from("selection_lines"),
            SharedString::from("highlighting"),
            SharedString::from("unfocused_selections"),
            SharedString::from("focus_bar"),
            SharedString::from("character_mode"),
            SharedString::from("token_mode"),
            SharedString::from("line_mode"),
            SharedString::from("open_file"),
            SharedString::from("save_file"),
            SharedString::from("set_language"),
            SharedString::from("find_replace"),
            SharedString::from("start"),
            SharedString::from("end"),
            SharedString::from("extend_start"),
            SharedString::from("extend_end"),
            SharedString::from("add_selection"),
            SharedString::from("select_next"),
            SharedString::from("focus_next"),
            SharedString::from("remove"),
            SharedString::from("delete"),
            SharedString::from("delete_line"),
            SharedString::from("zoom_in"),
            SharedString::from("zoom_out"),
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
            action_box: ComboBox::new("action", 0, false, false, actions),
        }
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {

        if let Action::Action = action {
            return (true, Some(false));
        }

        return self.action_box.handle_action(context, action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.action_box.add_character(character);
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, size: Vector2f, offset: Vector2f) {
        self.action_box.draw(framebuffer, context, size, offset, true);
    }
}
