use kami::*;
use sfml::graphics::*;
use sfml::system::Vector2f;
use context::{ Context, Action };

use super::{ ComboBox, ComboSelection };
use super::super::get_directory_entries;

pub struct FileBox {
    pub combobox: ComboBox,
    pub directories: usize,
    pub displacement: usize,
}

impl FileBox {

    pub fn new(description: &'static str, displacement: usize, allow_unknown: bool) -> Self {

        let entries = match get_directory_entries(&SharedString::from("./")) {
            Status::Success(entries) => entries,
            Status::Error(..) => Vec::new(),
        };

        Self {
            combobox: ComboBox::new(description, displacement, allow_unknown, true, entries),
            displacement: displacement,
            directories: 0,
        }
    }

    pub fn get(&self) -> SharedString {
        return self.combobox.get();
    }

    fn update_entries(&mut self, path: &SharedString) {

        let directories = self.combobox.get().position(&SharedString::from("/")).len();
        let complete_path = match directories {
            0 => format_shared!("./{}", path),
            _other => path.clone(),
        };

        match get_directory_entries(&complete_path) {
            Status::Success(entries) => self.combobox.variants = entries,
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

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {

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

        let return_value = self.combobox.handle_action(context, action);

        match action {

            Action::Confirm => self.check_directories(),

            Action::Remove => self.check_directories(),

            Action::Clear => self.check_directories(),

            Action::Paste => self.check_directories(),

            Action::Cut => self.check_directories(),

            _other => { },
        }

        return return_value;
    }

    pub fn add_character(&mut self, character: Character) {
        self.combobox.add_character(character);
        self.check_directories();
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, size: Vector2f, offset: Vector2f, focused: bool) {
        self.combobox.draw(framebuffer, context, size, offset, focused);
    }
}
