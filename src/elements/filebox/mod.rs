mod item;

use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::DialogueTheme;
use elements::{ ComboBox, ComboSelection, ComboItem };
use dialogues::DialogueStatus;
use interface::InterfaceContext;
use managers::LanguageManager;
use super::super::get_directory_entries;

use self::item::FileItem;

pub struct FileBox {
    combobox: ComboBox<FileItem>,
    directories: usize,
    show_hidden_files: bool,
}

impl FileBox {

    pub fn new(language_manager: &mut LanguageManager, description: &'static str, displacement: usize, allow_unknown: bool) -> Self {
        Self {
            combobox: ComboBox::new(language_manager, description, displacement, allow_unknown, Vec::new()),
            directories: 0,
            show_hidden_files: true,
        }
    }

    pub fn get_text(&self) -> SharedString {
        return self.combobox.get_text();
    }

    fn sort_entries(mut entries: Vec<SharedString>, path: &SharedString, show_hidden_files: bool) -> Vec<SharedString> {

        let mut index = 0;
        let mut remaining = entries.len();

        while index < remaining {
            if entries[index][0] == Character::from_char('.') {
                let entry = entries.remove(0);

                if show_hidden_files {
                    entries.push(entry);
                }

                remaining -= 1;
            } else {
                index += 1;
            }
        }

        for character in path.chars() {
            if *character != Character::from_char('.') && *character != Character::from_char('/') {
                return entries;
            }
        }

        if path[0] == Character::from_char('/') {
            return entries;
        }

        entries.push(SharedString::from("../"));
        return entries;
    }

    fn directory_count(&self) -> usize {
        return self.combobox.get_text().position(&SharedString::from("/")).len();
    }

    pub fn reload(&mut self, language_manager: &mut LanguageManager) {
        self.update_entries(language_manager, self.get_combined());
        self.directories = self.directory_count();
    }

    fn update_entries(&mut self, language_manager: &mut LanguageManager, path: SharedString) {
        let complete_path = match self.directory_count() {
            0 => format_shared!("./{}", path),
            _other => path.clone(),
        };

        if let Status::Success(entries) = get_directory_entries(&complete_path) {
            let sorted_entries = Self::sort_entries(entries, &complete_path, self.show_hidden_files);
            let items = sorted_entries.into_iter().map(|file_name| FileItem::new(file_name.clone(), &path)).collect();
            self.combobox.set_items(items);
            return;
        }

        self.combobox.clear(language_manager);
        self.combobox.reset_selection();
    }

    fn get_combined(&self) -> SharedString {
        let original = self.combobox.get_original();
        let positions = original.position(&SharedString::from("/"));

        match positions.is_empty() {
            true => SharedString::new(),
            false => original.slice(0, positions[positions.len() - 1]),
        }
    }

    fn check_directories(&mut self, language_manager: &mut LanguageManager) {
        let directories = self.combobox.get_text().position(&SharedString::from("/")).len();

        if self.directories != directories {
            self.update_entries(language_manager, self.get_combined());
            self.directories = directories;
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {

        if let Action::Confirm = action {
            if let ComboSelection::Item(index, _original) = self.combobox.get_selection() {
                let valid_entries = self.combobox.valid_items();

                if *valid_entries[index].display_name().chars().last().unwrap() == Character::from_char('/') {
                    self.combobox.reset_selection();
                    self.check_directories(language_manager);
                    return DialogueStatus::handled();
                }
            }
        }

        if let Action::RemoveSection = action {
            let text = self.combobox.get_text();
            let mut positions = text.position(&SharedString::from("/"));

            if let Some(position) = positions.last() {
                if *position == text.len() - 1 {
                    positions.remove(positions.len() - 1);
                }
            }

            if !positions.is_empty() {
                let last_position = positions.last().cloned().unwrap();
                let sliced_text = text.slice(0, last_position);
                self.combobox.set_text(language_manager, sliced_text);
            } else {
                self.combobox.clear(language_manager);
            }

            self.combobox.reset_selection();
            self.check_directories(language_manager);
            return DialogueStatus::handled();
        }

        if let Action::ToggleHiddenFiles = action {
            self.show_hidden_files = !self.show_hidden_files;

            self.update_entries(language_manager, self.get_combined());
            return DialogueStatus::handled();
        }

        let return_value = self.combobox.handle_action(interface_context, language_manager, action);

        //if action.modifies_text() {
        //    self.check_directories();
        //}

        match action {

            Action::Confirm => self.check_directories(language_manager),

            Action::FocusNext => self.check_directories(language_manager),

            Action::Remove => self.check_directories(language_manager),

            Action::Delete => self.check_directories(language_manager),

            Action::DeleteLine => self.check_directories(language_manager),

            Action::Paste => self.check_directories(language_manager),

            Action::Cut => self.check_directories(language_manager),

            _other => { },
        }

        return return_value;
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.combobox.add_character(language_manager, character);
        self.check_directories(language_manager);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.combobox.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme, focused: bool) {
        self.combobox.render(framebuffer, interface_context, theme, focused);
    }
}
