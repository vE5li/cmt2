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

use self::item::ThemeItem;

pub struct ThemeDialogue {
    combobox: ComboBox<ThemeItem>,
}

impl ThemeDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, "theme name", 0, false, Vec::new()),
        }
    }

    pub fn open(&mut self, language_manager: &mut LanguageManager) -> DialogueMode {
        self.update_items(language_manager);
        self.clear(language_manager);
        return DialogueMode::Theme;
    }

    fn update_items(&mut self, language_manager: &mut LanguageManager) {

        if let Status::Success(entries) = get_directory_entries(&format_shared!("/home/.config/poet/themes/")) {
            let items = entries.into_iter().map(|file_name| ThemeItem::new(file_name)).collect();
            self.combobox.set_items(items);
            return;
        }

        self.combobox.clear(language_manager);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {
            Action::Theme => return DialogueStatus::handled(),
            action => return self.combobox.handle_action(interface_context, language_manager, action),
        }
    }

    pub fn get_value(&self) -> SharedString {
        return self.combobox.get_value();
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
