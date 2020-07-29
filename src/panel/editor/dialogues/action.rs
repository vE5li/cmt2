use kami::*;
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
            SharedString::from("focus_left"),
            SharedString::from("focus_right"),
            SharedString::from("character_mode"),
            SharedString::from("token_mode"),
            SharedString::from("line_mode"),
            SharedString::from("open_file"),
            SharedString::from("set_language"),
            SharedString::from("find_replace"),
            SharedString::from("start"),
            SharedString::from("end"),
            SharedString::from("add_selection"),
            SharedString::from("focus_next"),
            SharedString::from("abort"),
            SharedString::from("confirm"),
            SharedString::from("remove"),
            SharedString::from("delete"),
            SharedString::from("clear"),
            SharedString::from("zoom_in_panel"),
            SharedString::from("zoom_out_panel"),
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
