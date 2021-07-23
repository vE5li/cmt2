mod item;

use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::DialogueTheme;
use elements::ComboBox;
use dialogues::{ DialogueMode, DialogueStatus };
use managers::LanguageManager;
use interface::InterfaceContext;

use self::item::LanguageItem;

pub struct LanguageDialogue {
    combobox: ComboBox<LanguageItem>,
}

impl LanguageDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, "language name", 0, false, Vec::new()),
        }
    }

    pub fn open(&mut self, language_manager: &mut LanguageManager, recommendation: &SharedString) -> DialogueMode {
        self.update_items(language_manager, recommendation);
        self.clear(language_manager);
        return DialogueMode::Language;
    }

    fn update_items(&mut self, language_manager: &mut LanguageManager, recommendation: &SharedString) {

        if let Status::Success(entries) = get_directory_entries(&format_shared!("/home/.config/poet/languages/")) {
            let items = entries.into_iter().map(|file_name| LanguageItem::new(file_name, recommendation)).collect();
            self.combobox.set_items(items);
            return;
        }

        self.combobox.clear(language_manager);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {
            Action::Language => return DialogueStatus::handled(),
            action => return self.combobox.handle_action(interface_context, language_manager, action),
        }
    }

    pub fn get_text(&self) -> SharedString {
        return self.combobox.get_text();
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
