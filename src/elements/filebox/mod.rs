use seamonkey::*;
use sfml::graphics::*;
use sfml::system::Vector2f;
use input::Action;

use elements::{ ComboBox, ComboSelection };
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use system::LanguageManager;
use super::super::get_directory_entries;

pub struct FileBox {
    pub combobox: ComboBox,
    pub directories: usize,
    pub displacement: usize,
    pub show_hidden_files: bool,
}

impl FileBox {

    pub fn new(language_manager: &mut LanguageManager, description: &'static str, displacement: usize, allow_unknown: bool) -> Self {

        let entries = match get_directory_entries(&SharedString::from("./")) {
            Status::Success(entries) => Self::entries_with_parent(entries, &SharedString::from("./"), true),
            Status::Error(..) => Vec::new(),
        };

        Self {
            combobox: ComboBox::new(language_manager, description, displacement, allow_unknown, true, entries),
            displacement: displacement,
            directories: 0,
            show_hidden_files: true,
        }
    }

    pub fn get(&self) -> SharedString {
        return self.combobox.get();
    }

    fn entries_with_parent(mut entries: Vec<SharedString>, path: &SharedString, show_hidden_files: bool) -> Vec<SharedString> {

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

    fn update_entries(&mut self, path: &SharedString) {
        let directories = self.combobox.get().position(&SharedString::from("/")).len();
        let complete_path = match directories {
            0 => format_shared!("./{}", path),
            _other => path.clone(),
        };

        match get_directory_entries(&complete_path) {
            Status::Success(entries) => self.combobox.variants = Self::entries_with_parent(entries, &complete_path, self.show_hidden_files),
            Status::Error(..) => self.combobox.variants.clear(),
        }
    }

    fn check_directories(&mut self) {
        let directories = self.combobox.get().position(&SharedString::from("/")).len();
        if self.directories != directories {
            self.directories = directories;

            match directories {
                0 => self.update_entries(&SharedString::new()),
                _other => self.update_entries(&self.combobox.get_combined(&SharedString::new())),
            }
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

        if let Action::Confirm = action {
            if let ComboSelection::Variant(index, _original) = self.combobox.selection.clone() {
                let valid_entries = self.combobox.valid_variants();

                if *valid_entries[index].chars().last().unwrap() == Character::from_char('/') {
                    self.combobox.selection = ComboSelection::TextBox;
                    self.check_directories();
                    return (true, None);
                }
            }
        }

        if let Action::RemoveSection = action {
            let text = self.combobox.get();
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

            self.combobox.selection = ComboSelection::TextBox;
            self.check_directories();
            return (true, None);
        }

        if let Action::ToggleHiddenFiles = action {
            self.show_hidden_files = !self.show_hidden_files;

            self.update_entries(&self.combobox.get_combined(&SharedString::new()));
            // cap selection (otherwise bad)
            return (true, None);
        }

        let return_value = self.combobox.handle_action(interface_context, language_manager, action);

        //if action.modifies_text() {
        //    self.check_directories();
        //}

        match action {

            Action::Confirm => self.check_directories(),

            Action::FocusNext => self.check_directories(),

            Action::Remove => self.check_directories(),

            Action::Delete => self.check_directories(),

            Action::DeleteLine => self.check_directories(),

            Action::Paste => self.check_directories(),

            Action::Cut => self.check_directories(),

            _other => { },
        }

        return return_value;
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.combobox.add_character(language_manager, character);
        self.check_directories();
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.combobox.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme, focused: bool) {
        self.combobox.render(framebuffer, interface_context, theme, focused);
    }
}
