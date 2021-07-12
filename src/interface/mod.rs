mod context;
mod theme;
mod vector;

use seamonkey::*;

use std::cmp::{ min, max };

use sfml::graphics::*;
use sfml::system::Vector2f;

use elements::*;
use dialogues::*;
use system::{ ResourceManager, Filebuffer };
use input::Action;

pub use self::context::InterfaceContext;
pub use self::vector::Vector4f;
pub use self::theme::*;

macro_rules! handle_return {
    ($expression: expr) => ({
        $expression;
        return None;
    })
}

macro_rules! confirm_or_error {
    ($interface: expr, $expression: expr) => ({
        match $expression {
            Status::Success(value) => value,
            Status::Error(error) => {
                $interface.set_error_state(error);
                $interface.set_language_dialogue.language_box.clear();
                return None;
            }
        }
    })
}

pub struct Interface {
    file_name: SharedString,
    textbuffer: Textbuffer,
    textbuffer_context: TextbufferContext,
    size: Vector2f,
    dialogue_mode: DialogueMode,
    open_file_dialogue: OpenFileDialogue,
    loaded_buffers_dialogue: LoadedBuffersDialogue,
    set_language_dialogue: SetLanguageDialogue,
    find_replace_dialogue: FindReplaceDialogue,
    action_dialogue: ActionDialogue,
    error_message: Option<SharedString>,
    popup: Popup,
}

impl Interface {

    pub fn new(resource_manager: &mut ResourceManager) -> Status<Self> {
        let new_name = format!("<unnamed {}>", resource_manager.next_index());
        resource_manager.filebuffers.insert(String::from(&new_name), Filebuffer::new(SharedString::from("\n")));

        success!(Self {
            file_name: SharedString::from(&new_name),
            textbuffer: Textbuffer::new(Vector2f::new(400., 50.), Vector2f::new(0., 0.), '\n', true, true, true),
            textbuffer_context: TextbufferContext::from(),
            size: Vector2f::new(0.0, 0.0),
            dialogue_mode: DialogueMode::None,
            open_file_dialogue: OpenFileDialogue::new(),
            loaded_buffers_dialogue: LoadedBuffersDialogue::new(),
            set_language_dialogue: SetLanguageDialogue::new(),
            find_replace_dialogue: FindReplaceDialogue::new(),
            action_dialogue: ActionDialogue::new(),
            error_message: None,
            popup: Popup::new(),
        })
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &InterfaceTheme) {
        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_number_offset = match self.textbuffer_context.line_numbers {
            true => theme.textbuffer_theme.line_number_width as f32 * character_scaling + theme.textbuffer_theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };

        let left_position = line_number_offset + theme.textbuffer_theme.offset.x * interface_context.font_size as f32;
        let right_position = theme.textbuffer_theme.offset.x * interface_context.font_size as f32;
        let top_position = theme.textbuffer_theme.offset.y * interface_context.font_size as f32;
        let size = Vector2f::new(self.size.x - left_position - right_position, self.size.y - top_position);
        let position = Vector2f::new(left_position, top_position);

        self.open_file_dialogue.update_layout(interface_context, &theme.dialogue_theme, size, position);
        self.loaded_buffers_dialogue.update_layout(interface_context, &theme.dialogue_theme, size, position);
        self.set_language_dialogue.update_layout(interface_context, &theme.dialogue_theme, size, position);
        self.find_replace_dialogue.update_layout(interface_context, &theme.dialogue_theme, size, position);
        self.action_dialogue.update_layout(interface_context, &theme.dialogue_theme, size, position);

        self.popup.update_layout(size, position);
    }

    pub fn resize(&mut self, size: Vector2f) {
        self.size = size;
        self.textbuffer.resize(size);
    }

    pub fn new_file(&mut self, resource_manager: &mut ResourceManager) -> Status<()> {

        let new_name = format!("<unnamed {}>", resource_manager.next_index());
        resource_manager.filebuffers.insert(String::from(&new_name), Filebuffer::new(SharedString::from("\n")));
        self.file_name = SharedString::from(&new_name);

        // update language

        //return self.textbuffer.set_text(SharedString::from("\n"));
        return success!(());
    }

    pub fn open_file(&mut self, resource_manager: &mut ResourceManager, file_name: SharedString) -> Status<()> {

        if file_name == self.file_name {
            return success!(());
        }

        let mut text = confirm!(read_file(&file_name));
        let string_file_name = file_name.serialize();

        if text.is_empty() || !text[text.len() - 1].is_newline() {
            text.push(Character::from_char('\n'));
        }

        if resource_manager.filebuffers.get(&string_file_name).is_none() {
            resource_manager.filebuffers.insert(string_file_name.clone(), Filebuffer::new(text.clone()));
        }

        let current_file_name = self.file_name.serialize();
        let current_file_length = resource_manager.filebuffers.get(&current_file_name).unwrap().length();

        if self.file_name[0] == Character::from_char('<') && current_file_length <= 1 {
            resource_manager.filebuffers.remove(&current_file_name);
        }

        // update language
        //return self.textbuffer.set_text(text);
        return success!(());
    }

    pub fn save_file(&mut self) {

        //let file_name = match &self.file_name {
        //    Some(file_name) => file_name,
        //    None => {
        //        self.set_error_state(Error::Message(string!("cannot save file without file name (yet)")));
        //        return;
        //    },
        //};

        //if let Status::Error(error) = write_file(&self.file_name, &self.textbuffer.get_text()) {
        //    self.set_error_state(error);
        //}
    }

    pub fn scroll_up(&mut self) {
        self.textbuffer.scroll_up(&self.textbuffer_context);
    }

    pub fn scroll_down(&mut self) {
        self.textbuffer.scroll_down(&self.textbuffer_context);
    }

    pub fn open_buffer(&mut self, resource_manager: &mut ResourceManager, file_name: SharedString) -> Option<Action> {
        let string_file_name = file_name.serialize();

        if file_name != self.file_name {

            if resource_manager.filebuffers.get(&string_file_name).is_none() {
                self.open_file(resource_manager, file_name.clone());
            }

            let filebuffer = resource_manager.filebuffers.get_mut(&string_file_name).unwrap();
            self.textbuffer.set_text_without_save(filebuffer, filebuffer.get_text());
            self.file_name = file_name;
        }

        self.dialogue_mode = DialogueMode::None;
        return None;
    }

    pub fn history_catch_up(&mut self, interface_context: &InterfaceContext, resource_manager: &mut ResourceManager) -> bool {
        let filebuffer = resource_manager.filebuffers.get_mut(&self.file_name.serialize()).unwrap();
        return self.textbuffer.history_catch_up(filebuffer);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, resource_manager: &mut ResourceManager, action: Action) -> Option<Action> {

        if self.error_message.is_some() {
            self.error_message = None;
        }

        let unhandled_action = match self.dialogue_mode.clone() {

            DialogueMode::None => {
                let filebuffer = resource_manager.filebuffers.get_mut(&self.file_name.serialize()).unwrap();
                self.textbuffer.handle_action(&self.textbuffer_context, filebuffer, action)
            },

            DialogueMode::OpenFile => {
                let (action_handled, status) = self.open_file_dialogue.handle_action(interface_context, action);

                if let Some(completed) = status {
                    if completed {
                        let file_name = self.open_file_dialogue.file_name_box.get();
                        return self.open_buffer(resource_manager, file_name);
                    }

                    self.dialogue_mode = DialogueMode::None;
                    return None;
                }

                match action_handled {
                    true => return None,
                    false =>  return Some(action),
                }
            },

            DialogueMode::LoadedBuffers => {
                let (action_handled, status) = self.loaded_buffers_dialogue.handle_action(interface_context, action);

                if let Some(completed) = status {
                    if completed {
                        let file_name = self.loaded_buffers_dialogue.buffers_box.get();
                        return self.open_buffer(resource_manager, file_name);
                    }

                    self.dialogue_mode = DialogueMode::None;
                    return None;
                }

                match action_handled {
                    true => return None,
                    false =>  return Some(action),
                }
            },

            DialogueMode::SetLanguage => {
                let (action_handled, status) = self.set_language_dialogue.handle_action(interface_context, action);

                if let Some(completed) = status {
                    panic!();
                    //if completed && self.language != self.set_language_dialogue.language_box.get() {
                    //    let new_language = self.set_language_dialogue.language_box.get();

                    //    match Self::load_language(&new_language) {

                    //        Status::Success(tokenizer) => {
                    //            self.language = new_language;
                    //            self.tokenizer = tokenizer;
                    //            self.parse();
                    //        },

                    //        Status::Error(error) => {
                    //            self.set_error_state(error);
                    //            self.set_language_dialogue.language_box.clear();
                    //        }
                    //    }

                    //    return None;
                    //}

                    self.set_language_dialogue.language_box.clear();
                    self.dialogue_mode = DialogueMode::None;
                }

                match action_handled {
                    true => return None,
                    false =>  return Some(action),
                }
            },

            DialogueMode::FindReplace(selections) => {

                let (action_handled, status) = self.find_replace_dialogue.handle_action(action);

                if let Some(completed) = status {
                    panic!();

                    //if completed && self.selections.len() != 0 {

                        //let find = self.find_replace_dialogue.find_box.get();
                        //let replace = self.find_replace_dialogue.replace_box.get();
                        //self.textbuffer = self.textbuffer.replace(&find, &replace);

                        //if find.len() > replace.len() {

                        //    let difference = find.len() - replace.len();
                        //    for index in 0..self.selections.len() {
                        //        self.selections[index].length -= difference;
                        //        self.selections[index].index -= difference * index;
                        //    }
                        //} else if find.len() < replace.len() {

                        //    let difference = replace.len() - find.len();
                        //    for index in 0..self.selections.len() {
                        //        self.selections[index].length += difference;
                        //        self.selections[index].index += difference * index;
                        //    }
                        //}

                        //self.character_mode();
                        //self.parse();
                    //} else {

                    //    self.selections = selections.clone();
                    //}

                    self.dialogue_mode = DialogueMode::None;
                    return None;
                }

                //if action_handled {
                //    self.update_find_replace();
                //}

                match action_handled {
                    true => return None,
                    false =>  return Some(action),
                }
            },

            DialogueMode::Action => {
                let (action_handled, status) = self.action_dialogue.handle_action(interface_context, action);

                if let Some(completed) = status {
                    let literal = self.action_dialogue.action_box.get();
                    self.action_dialogue.action_box.clear();
                    self.dialogue_mode = DialogueMode::None;

                    if completed {
                        let action = confirm_or_error!(self, Action::from_literal(&literal));
                        return self.handle_action(interface_context, resource_manager, action);
                    }
                }

                match action_handled {
                    true => return None,
                    false =>  return Some(action),
                }
            },
        };

        if let Some(action) = unhandled_action {
            match action {
                Action::NewFile => handle_return!(self.new_file(resource_manager)),
                Action::OpenFile => handle_return!(self.open_open_file_dialogue()),
                Action::LoadedBuffers => handle_return!(self.open_loaded_buffers_dialogue(resource_manager)),
                Action::SaveFile => handle_return!(self.save_file()),
                //Action::SaveAllFiles => handle_me_in_core,
                Action::SetLanguage => handle_return!(self.open_set_language_dialogue()),
                Action::FindReplace => handle_return!(self.open_find_replace_dialogue()),
                Action::Action => handle_return!(self.open_action_dialogue()),
                unhandled => return Some(unhandled),
            }
        }

        return None;
    }

    pub fn set_error_state(&mut self, error: Error) {
        let message = error.display(&None, &map!());
        self.error_message = Some(message);
        self.dialogue_mode = DialogueMode::None;
    }

    fn open_open_file_dialogue(&mut self) {

        self.dialogue_mode = DialogueMode::OpenFile;
    }

    fn open_loaded_buffers_dialogue(&mut self, resource_manager: &ResourceManager) {
        self.dialogue_mode = DialogueMode::LoadedBuffers;
        self.loaded_buffers_dialogue.update_variants(resource_manager);
        self.loaded_buffers_dialogue.clear();
    }

    fn open_set_language_dialogue(&mut self) {

        self.dialogue_mode = DialogueMode::SetLanguage;
        self.set_language_dialogue.clear();
    }

    fn open_find_replace_dialogue(&mut self) {

        self.find_replace_dialogue.reset();
        //self.dialogue_mode = DialogueMode::FindReplace(self.selections.clone());

        //self.update_find_replace();
    }

    fn open_action_dialogue(&mut self) {

        self.dialogue_mode = DialogueMode::Action;
    }

    pub fn add_character(&mut self, interface_context: &InterfaceContext, resource_manager: &mut ResourceManager, character: Character) {

        if self.error_message.is_some() {
            self.error_message = None;
        }

        match self.dialogue_mode.clone() {

            DialogueMode::OpenFile => self.open_file_dialogue.add_character(character),

            DialogueMode::LoadedBuffers => self.loaded_buffers_dialogue.add_character(character),

            DialogueMode::SetLanguage => self.set_language_dialogue.add_character(character),

            DialogueMode::FindReplace(..) => {

                self.find_replace_dialogue.add_character(character);
                //self.update_find_replace();
            },

            DialogueMode::Action => self.action_dialogue.add_character(character),

            DialogueMode::None => {
                let filebuffer = resource_manager.filebuffers.get_mut(&self.file_name.serialize()).unwrap();
                self.textbuffer.add_character(&self.textbuffer_context, filebuffer, character);
            },
        }
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &InterfaceTheme, resource_manager: &ResourceManager, focused: bool) {

        let mut filebuffer = resource_manager.filebuffers.get(&self.file_name.serialize()).unwrap();
        self.textbuffer.render(framebuffer, interface_context, &self.textbuffer_context, &theme.textbuffer_theme, filebuffer, interface_context.font_size as f32, focused);

        if let Some(error_message) = &self.error_message {
            self.popup.render(framebuffer, interface_context, &theme.message_theme.error_theme, error_message);
        }

        match &self.dialogue_mode {
            DialogueMode::OpenFile => self.open_file_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),
            DialogueMode::LoadedBuffers => self.loaded_buffers_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),
            DialogueMode::SetLanguage => self.set_language_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),
            DialogueMode::FindReplace(..) => self.find_replace_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),
            DialogueMode::Action => self.action_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),
            DialogueMode::None => { },
        }
    }
}
