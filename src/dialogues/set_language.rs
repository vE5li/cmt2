use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use system::LanguageManager;
use elements::ComboBox;
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use input::Action;

pub struct SetLanguageDialogue {
    pub language_box: ComboBox,
}

impl SetLanguageDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            language_box: ComboBox::new(language_manager, "language name", 0, false, false, vec![SharedString::from("cipher"), SharedString::from("c++"), SharedString::from("default"), SharedString::from("doofenshmirtz"), SharedString::from("entleman"), SharedString::from("none"), SharedString::from("rust"), SharedString::from("seamonkey")]),
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

        if let Action::SetLanguage = action {
            return (true, Some(false));
        }

        return self.language_box.handle_action(interface_context, language_manager, action);
    }

    pub fn clear(&mut self, language_manager: &mut LanguageManager) {
        self.language_box.clear(language_manager);
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.language_box.add_character(language_manager, character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.language_box.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.language_box.render(framebuffer, interface_context, theme, true);
    }
}
