use kami::*;
use context::{ Context, Action };
use sfml::graphics::*;
use sfml::system::Vector2f;
use super::super::FileBox;

pub struct OpenFileDialogue {
    pub file_name_box: FileBox,
}

impl OpenFileDialogue {

    pub fn new() -> Self {
        Self {
            file_name_box: FileBox::new("file path", 0, false),
        }
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {

        if let Action::OpenFile = action {
            return (true, Some(false));
        }

        return self.file_name_box.handle_action(context, action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.file_name_box.add_character(character);
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, size: Vector2f, offset: Vector2f) {
        self.file_name_box.draw(framebuffer, context, size, offset, true);
    }
}
