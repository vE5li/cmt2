use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::DialogueTheme;
use elements::FileBox;
use dialogues::{ DialogueMode, DialogueStatus };
use interface::InterfaceContext;
use managers::LanguageManager;

pub struct OpenDialogue {
    filebox: FileBox,
}

impl OpenDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            filebox: FileBox::new(language_manager, "file path", 0, false),
        }
    }

    pub fn open(&mut self, language_manager: &mut LanguageManager) -> DialogueMode {
        self.filebox.reload(language_manager);
        return DialogueMode::Open;
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {
            Action::Open => return DialogueStatus::handled(),
            action => return self.filebox.handle_action(interface_context, language_manager, action),
        }
    }

    pub fn get_text(&self) -> SharedString {
        return self.filebox.get_text();
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
