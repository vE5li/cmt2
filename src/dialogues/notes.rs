use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use system::{ LanguageManager, Filebuffer };
use elements::ComboBox;
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use input::Action;

pub struct NotesDialogue {
    combobox: ComboBox,
}

impl NotesDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, "note", 0, false, false, Vec::new()),
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

        if let Action::Notes = action {
            return (true, Some(false));
        }

        return self.combobox.handle_action(interface_context, language_manager, action);
    }

    pub fn update_variants(&mut self, filebuffer: &Filebuffer) {
        let variants = filebuffer.get_notes().iter().map(|note| format_shared!("{}: {}", note.kind.serialize(), note.message)).collect();
        self.combobox.variants = variants;
    }

    pub fn get(&self) -> SharedString {
        return self.combobox.get();
    }

    pub fn clear(&mut self, language_manager: &mut LanguageManager) {
        self.combobox.clear(language_manager);
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
