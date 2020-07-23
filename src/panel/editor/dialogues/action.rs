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
            VectorString::from("focus_left"),
            VectorString::from("focus_right"),
            VectorString::from("character_mode"),
            VectorString::from("token_mode"),
            VectorString::from("line_mode"),
            VectorString::from("open_file"),
            VectorString::from("set_language"),
            VectorString::from("find_replace"),
            VectorString::from("start"),
            VectorString::from("end"),
            VectorString::from("add_selection"),
            VectorString::from("focus_next"),
            VectorString::from("abort"),
            VectorString::from("confirm"),
            VectorString::from("remove"),
            VectorString::from("delete"),
            VectorString::from("clear"),
            VectorString::from("zoom_in_panel"),
            VectorString::from("zoom_out_panel"),
            VectorString::from("left"),
            VectorString::from("right"),
            VectorString::from("up"),
            VectorString::from("down"),
            VectorString::from("extend_left"),
            VectorString::from("extend_right"),
            VectorString::from("extend_up"),
            VectorString::from("extend_down"),
            VectorString::from("move_left"),
            VectorString::from("move_right"),
            VectorString::from("move_up"),
            VectorString::from("move_down"),
            VectorString::from("copy"),
            VectorString::from("paste"),
            VectorString::from("cut"),
            VectorString::from("rotate"),
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
