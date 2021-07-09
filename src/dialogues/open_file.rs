use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use elements::FileBox;
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use input::Action;

pub struct OpenFileDialogue {
    pub file_name_box: FileBox,
}

impl OpenFileDialogue {

    pub fn new() -> Self {
        Self {
            file_name_box: FileBox::new("file path", 0, false),
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, action: Action) -> (bool, Option<bool>) {

        if let Action::OpenFile = action {
            return (true, Some(false));
        }

        return self.file_name_box.handle_action(interface_context, action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.file_name_box.add_character(character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.file_name_box.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.file_name_box.render(framebuffer, interface_context, theme, true);
    }
}
