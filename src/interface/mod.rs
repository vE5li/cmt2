mod context;
mod vector;

use seamonkey::*;

#[cfg(feature = "debug")]
use debug::*;

use std::cmp::{ min, max };
use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::InterfaceTheme;
use filebuffer::Filebuffer;
use elements::*;
use dialogues::*;
use managers::*;

pub use self::context::InterfaceContext;
pub use self::vector::Vector4f;

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
                return None;
            }
        }
    })
}

pub struct Interface {
    file_name: SharedString,
    textbuffer: Textbuffer,
    dialogue_mode: DialogueMode,
    open_file_dialogue: OpenDialogue,
    loaded_buffers_dialogue: FilebuffersDialogue,
    notes_dialogue: NotesDialogue,
    set_language_dialogue: LanguageDialogue,
    set_theme_dialogue: ThemeDialogue,
    find_replace_dialogue: ReplaceDialogue,
    action_dialogue: ActionDialogue,
    error_message: Option<SharedString>,
    popup: Popup,
}

impl Interface {

    pub fn new(filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, window_id: usize) -> Status<Self> {
        let language = SharedString::from("none");
        let new_name = format!("<unnamed {}>", filebuffer_manager.next_index());
        let filebuffer = Filebuffer::new(language_manager, language, SharedString::from("\n"));
        filebuffer_manager.insert(String::from(&new_name), filebuffer);

        success!(Self {
            file_name: SharedString::from(&new_name),
            textbuffer: Textbuffer::new(window_id, Vector2f::new(400., 50.), Vector2f::new(0., 0.), '\n'),
            dialogue_mode: DialogueMode::None,
            open_file_dialogue: OpenDialogue::new(language_manager),
            loaded_buffers_dialogue: FilebuffersDialogue::new(language_manager),
            notes_dialogue: NotesDialogue::new(language_manager),
            set_language_dialogue: LanguageDialogue::new(language_manager),
            set_theme_dialogue: ThemeDialogue::new(language_manager),
            find_replace_dialogue: ReplaceDialogue::new(language_manager),
            action_dialogue: ActionDialogue::new(language_manager),
            error_message: None,
            popup: Popup::new(),
        })
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, filebuffer_manager: &FilebufferManager, theme: &InterfaceTheme, size: Vector2f) {
        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.textbuffer_theme.line_number_width as f32 * character_scaling + theme.textbuffer_theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };

        let left_position = line_number_offset + theme.textbuffer_theme.offset.x * interface_context.font_size as f32;
        let right_position = theme.textbuffer_theme.offset.x * interface_context.font_size as f32;
        let top_position = theme.textbuffer_theme.offset.y * interface_context.font_size as f32;
        let dialogue_size = Vector2f::new(size.x - left_position - right_position, size.y - top_position);
        let position = Vector2f::new(left_position, top_position);

        let filebuffer = filebuffer_manager.get(&self.file_name.serialize());
        self.textbuffer.update_layout(interface_context, textbuffer_context, filebuffer, size);

        self.open_file_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.loaded_buffers_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.notes_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.set_language_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.set_theme_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.find_replace_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);
        self.action_dialogue.update_layout(interface_context, &theme.dialogue_theme, dialogue_size, position);

        self.popup.update_layout(dialogue_size, position);
    }

    pub fn new_file(&mut self, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager) -> Status<()> {

        #[cfg(feature = "debug")]
        let timer = Timer::new("new file");

        let language = SharedString::from("none");
        let new_name = format!("<unnamed {}>", filebuffer_manager.next_index());

        let mut filebuffer = Filebuffer::new(language_manager, language, SharedString::from("\n"));

        self.textbuffer.reset(&mut filebuffer);
        filebuffer_manager.insert(String::from(&new_name), filebuffer);

        self.file_name = SharedString::from(&new_name);

        #[cfg(feature = "debug")]
        timer.stop();

        // update language

        //return self.textbuffer.set_text(SharedString::from("\n"));
        return success!(());
    }

    pub fn save_file(&mut self, filebuffer_manager: &mut FilebufferManager) {

        #[cfg(feature = "debug")]
        let timer = Timer::new("save file");

        if self.file_name[0] == Character::from_char('<') {
            self.set_error_state(Error::Message(string!("cannot save file without file name (yet)")));
            return;
        }

        let current_text = filebuffer_manager.get(&self.file_name.serialize()).get_text();
        if let Status::Error(error) = write_file(&self.file_name, &current_text) {
            self.set_error_state(error);
        }

        #[cfg(feature = "debug")]
        timer.stop();
    }

    pub fn scroll_up(&mut self, textbuffer_context: &TextbufferContext) {
        self.textbuffer.scroll_up(textbuffer_context);
    }

    pub fn scroll_down(&mut self, filebuffer_manager: &FilebufferManager, textbuffer_context: &TextbufferContext) {
        let filebuffer = filebuffer_manager.get(&self.file_name.serialize());
        self.textbuffer.scroll_down(textbuffer_context, filebuffer);
    }

    pub fn open_buffer(&mut self, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, file_name: SharedString) {
        let string_file_name = file_name.serialize();

        if file_name != self.file_name {

            #[cfg(feature = "debug")]
            let timer = Timer::new_dynamic(format!("open buffer {}{}{}", magenta(), string_file_name, none()));

            if !filebuffer_manager.contains(&string_file_name) {

                #[cfg(feature = "debug")]
                let read_timer = Timer::new_dynamic(format!("read file {}{}{}", magenta(), string_file_name, none()));

                let language = SharedString::from("none");
                let mut text = display!(read_file(&file_name));

                if text.is_empty() || !text[text.len() - 1].is_newline() {
                    text.push(Character::from_char('\n'));
                }

                let filebuffer = Filebuffer::new(language_manager, language, text.clone());
                filebuffer_manager.insert(string_file_name.clone(), filebuffer);

                #[cfg(feature = "debug")]
                read_timer.stop();
            }

            let current_file_name = self.file_name.serialize();
            let current_file_length = filebuffer_manager.get(&current_file_name).length();

            if self.file_name[0] == Character::from_char('<') && current_file_length <= 1 {
                filebuffer_manager.remove(&current_file_name);
            }

            let filebuffer = filebuffer_manager.get_mut(&string_file_name);
            self.textbuffer.reset(filebuffer);
            self.file_name = file_name;

            #[cfg(feature = "debug")]
            timer.stop();
        }
    }

    pub fn history_catch_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer_manager: &mut FilebufferManager) -> bool {
        let filebuffer = filebuffer_manager.get_mut(&self.file_name.serialize());
        return self.textbuffer.history_catch_up(textbuffer_context, filebuffer);
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, action: Action, theme_name: &mut SharedString) -> Option<Action> {

        if self.error_message.is_some() {
            self.error_message = None;
        }

        let filebuffer = filebuffer_manager.get_mut(&self.file_name.serialize());

        let unhandled_action = match self.dialogue_mode.clone() {

            DialogueMode::None => self.textbuffer.handle_action(textbuffer_context, language_manager, filebuffer, action),

            DialogueMode::Open => {
                let status = self.open_file_dialogue.handle_action(interface_context, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    let file_name = self.open_file_dialogue.get_text();
                    self.open_buffer(filebuffer_manager, language_manager, file_name);
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Filebuffers => {
                let status = self.loaded_buffers_dialogue.handle_action(interface_context, filebuffer_manager, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    let file_name = self.loaded_buffers_dialogue.get_text();
                    self.open_buffer(filebuffer_manager, language_manager, file_name);
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Notes => {
                let status = self.notes_dialogue.handle_action(interface_context, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    let index = self.notes_dialogue.get_value();
                    self.textbuffer.jump_to_index(textbuffer_context, filebuffer, index);
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Language => {
                let status = self.set_language_dialogue.handle_action(interface_context, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    let language = self.set_language_dialogue.get_text();
                    let filebuffer = filebuffer_manager.get_mut(&self.file_name.serialize());
                    confirm_or_error!(self, filebuffer.set_language(language_manager, language));
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Theme => {
                let status = self.set_theme_dialogue.handle_action(interface_context, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    *theme_name = self.set_theme_dialogue.get_value();
                    return Some(Action::Reload);
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Replace(selections) => {

                let status = self.find_replace_dialogue.handle_action(language_manager, action);

                //if let Some(completed) = status {
                //    panic!();

                    //if completed && self.selections.len() != 0 {

                        //let find = self.find_replace_dialogue.find_box.get_text();
                        //let replace = self.find_replace_dialogue.replace_box.get_text();
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

                //    self.dialogue_mode = DialogueMode::None;
                //    return None;
                //}

                //if action_handled {
                //    self.update_find_replace();
                //}

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },

            DialogueMode::Action => {
                let status = self.action_dialogue.handle_action(interface_context, language_manager, action);

                if status.closed {
                    self.dialogue_mode = DialogueMode::None;
                }

                if status.completed {
                    let action = self.action_dialogue.get_value();
                    return self.handle_action(interface_context, textbuffer_context, filebuffer_manager, language_manager, action, theme_name);
                }

                match status.handled {
                    true => return None,
                    false => return Some(action),
                }
            },
        };

        if let Some(action) = unhandled_action {
            match action {

                Action::NewFile => handle_return!(self.new_file(filebuffer_manager, language_manager)),

                Action::Open => handle_return!(self.dialogue_mode = self.open_file_dialogue.open(language_manager)),

                Action::Filebuffers => handle_return!(self.dialogue_mode = self.loaded_buffers_dialogue.open(filebuffer_manager, language_manager)),

                Action::Notes => handle_return!(self.dialogue_mode = self.notes_dialogue.open(filebuffer, language_manager)),

                Action::SaveFile => handle_return!(self.save_file(filebuffer_manager)),

                //Action::SaveAllFiles => handle_me_in_core,

                Action::Theme => handle_return!(self.dialogue_mode = self.set_theme_dialogue.open(language_manager)),

                Action::Language => handle_return!(self.dialogue_mode = self.set_language_dialogue.open(language_manager, &SharedString::from("cipher"))), // load recommendation dynamically

                Action::Replace => handle_return!(self.dialogue_mode = self.find_replace_dialogue.open(language_manager, self.textbuffer.get_selections())),

                Action::Action => handle_return!(self.dialogue_mode = self.action_dialogue.open()),

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

    pub fn add_character(&mut self, textbuffer_context: &TextbufferContext, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, character: Character) {

        if self.error_message.is_some() {
            self.error_message = None;
        }

        match self.dialogue_mode.clone() {

            DialogueMode::Open => self.open_file_dialogue.add_character(language_manager, character),

            DialogueMode::Filebuffers => self.loaded_buffers_dialogue.add_character(language_manager, character),

            DialogueMode::Notes => self.notes_dialogue.add_character(language_manager, character),

            DialogueMode::Language => self.set_language_dialogue.add_character(language_manager, character),

            DialogueMode::Theme => self.set_theme_dialogue.add_character(language_manager, character),

            DialogueMode::Replace(..) => {
                self.find_replace_dialogue.add_character(language_manager, character);
                //self.update_find_replace();
            },

            DialogueMode::Action => self.action_dialogue.add_character(language_manager, character),

            DialogueMode::None => {
                let filebuffer = filebuffer_manager.get_mut(&self.file_name.serialize());
                self.textbuffer.add_character(textbuffer_context, language_manager, filebuffer, character);
            },
        }
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &InterfaceTheme, filebuffer_manager: &FilebufferManager, focused: bool) {

        let filebuffer = filebuffer_manager.get(&self.file_name.serialize());
        self.textbuffer.render(framebuffer, interface_context, textbuffer_context, &theme.textbuffer_theme, filebuffer, interface_context.font_size as f32, focused);

        if let Some(error_message) = &self.error_message {
            self.popup.render(framebuffer, interface_context, &theme.message_theme.error_theme, error_message);
        }

        match &self.dialogue_mode {

            DialogueMode::Open => self.open_file_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Filebuffers => self.loaded_buffers_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Notes => self.notes_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Language => self.set_language_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Theme => self.set_theme_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Replace(..) => self.find_replace_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::Action => self.action_dialogue.render(framebuffer, interface_context, &theme.dialogue_theme),

            DialogueMode::None => { },
        }
    }
}
