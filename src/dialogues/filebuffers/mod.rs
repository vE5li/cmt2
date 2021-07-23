mod item;

use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::{ DialogueTheme, ItemTheme, TextfieldTheme };
use elements::{ ComboBox, ComboItem };
use dialogues::{ DialogueMode, DialogueStatus };
use interface::InterfaceContext;
use managers::{ FilebufferManager, LanguageManager };

use self::item::BufferItem;

pub struct FilebuffersDialogue {
    combobox: ComboBox<BufferItem>,
}

impl FilebuffersDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, "recently opened files", 0, false, Vec::new()),
        }
    }

    pub fn open(&mut self, filebuffer_manager: &FilebufferManager, language_manager: &mut LanguageManager) -> DialogueMode {
        self.update_items(filebuffer_manager);
        self.clear(language_manager);
        return DialogueMode::Filebuffers;
    }

    fn update_items(&mut self, filebuffer_manager: &FilebufferManager) {
        let items = filebuffer_manager.iter().map(|(name, _)| BufferItem::new(name)).collect();
        self.combobox.set_items(items);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {

        if let Action::Filebuffers = action {
            return DialogueStatus::handled();
        }

        if let Action::Delete = action {
            if !self.combobox.is_textbox_focused() {
                let buffer_name = self.combobox.get_text();

                // make sure that buffer has no unsaved changes

                filebuffer_manager.remove(&buffer_name.serialize());
                self.combobox.remove_selected_item(interface_context, language_manager);
                return DialogueStatus::handled();
            }
        }

        return self.combobox.handle_action(interface_context, language_manager, action);
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
