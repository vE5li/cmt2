use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use system::LanguageManager;
use elements::FileBox;
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use input::Action;

pub struct OpenFileDialogue {
    filebox: FileBox,
}

impl OpenFileDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            filebox: FileBox::new(language_manager, "file path", 0, false),
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

        if let Action::OpenFile = action {
            return (true, Some(false));
        }

        return self.filebox.handle_action(interface_context, language_manager, action);
    }

    pub fn get(&self) -> SharedString {
        return self.filebox.get();
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.filebox.add_character(language_manager, character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.filebox.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.filebox.render(framebuffer, interface_context, theme, true);
    }
}
