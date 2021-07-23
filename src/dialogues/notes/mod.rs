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
use filebuffer::Filebuffer;

use self::item::NoteItem;

pub struct NotesDialogue {
    combobox: ComboBox<NoteItem>,
}

impl NotesDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, "note", 0, false, Vec::new()),
        }
    }

    pub fn open(&mut self, filebuffer: &Filebuffer, language_manager: &mut LanguageManager) -> DialogueMode {
        self.update_items(filebuffer);
        self.clear(language_manager);
        return DialogueMode::Notes;
    }

    fn update_items(&mut self, filebuffer: &Filebuffer) {
        let items = filebuffer.get_notes().iter().map(|note| NoteItem::new(note)).collect();
        self.combobox.set_items(items);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {
            Action::Notes => return DialogueStatus::handled(),
            action => return self.combobox.handle_action(interface_context, language_manager, action),
        }
    }

    pub fn get_value(&self) -> usize {
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
