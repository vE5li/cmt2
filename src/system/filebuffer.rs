use seamonkey::*;

use system::{ BufferAction, LanguageManager, History };
use elements::{ SelectionMode, Token };

pub fn length_from_position(position: Vec<Position>) -> usize {
    return position.iter().map(|position| position.length).sum();
}

#[derive(Clone)]
pub struct Filebuffer {
    text: SharedString,
    history: History,
    history_index: usize,
    pub tokens: Vec<Token>,
    language: SharedString,
}

impl Filebuffer {

    pub fn new(language_manager: &mut LanguageManager, language: SharedString, text: SharedString) -> Self {
        let tokens = display!(Self::tokenize(language_manager, &language, &text));

        return Self {
            text: text,
            history: History::new(),
            history_index: 0,
            tokens: tokens,
            language: language,
        }
    }

    fn tokenize(language_manager: &mut LanguageManager, language: &SharedString, text: &SharedString) -> Status<Vec<Token>> {

        let tokenizer = confirm!(language_manager.get_load(language));
        let (mut token_stream, registry, notes) = display!(tokenizer.tokenize(text.clone(), None, true));
        let mut tokens = Vec::new();
        let mut offset = 0;

        for token in token_stream.into_iter() {
            let length = length_from_position(token.position);
            tokens.push(Token::new(token.token_type, offset, length));
            offset += length;
        }

        return success!(tokens);
    }

    pub fn retokenize(&mut self, language_manager: &mut LanguageManager) -> Status<()> {
        self.tokens = confirm!(Self::tokenize(language_manager, &self.language, &self.text));
        return success!(());
    }

    pub fn set_language(&mut self, language_manager: &mut LanguageManager, language: SharedString) -> Status<()> {
        if self.language == language {
            return success!(());
        }

        self.language = language;
        return self.retokenize(language_manager);
    }

    pub fn get_token_from_index(&self, index: usize) -> Token {
        for token in &self.tokens {
            if token.index + token.length > index {
                return token.clone();
            }
        }
        panic!("index from token failed");
    }

    fn insert_text_raw(&mut self, index: usize, text: &SharedString) {
        for offset in (0..text.len()).rev() {
            match offset == self.text.len() {
                true => self.text.push(text[offset]),
                false => self.text.insert(index, text[offset]),
            }
        }
    }

    fn remove_text_raw(&mut self, index: usize, length: usize) {
        for _ in 0..length {
            self.text.remove(index);
        }
    }

    pub fn last_buffer_index(&self) -> usize {
        match self.text.is_empty() {
            true => return 0,
            false => return self.text.len() - 1,
        }
    }

    fn advance(&mut self, offset: usize) -> usize {
        self.history_index += offset;
        return self.history_index;
    }

    pub fn insert_text(&mut self, window_id: usize, index: usize, text: SharedString, combine: bool) -> usize {
        self.insert_text_raw(index, &text);
        self.history.pop_until(self.history_index);
        self.history.insert_text(window_id, index, text, combine);
        return self.advance(1);
    }

    pub fn remove_text(&mut self, window_id: usize, index: usize, length: usize, combine: bool) -> usize {
        let removed_text = self.text.slice(index, index + length - 1);
        self.remove_text_raw(index, length);
        self.history.pop_until(self.history_index);
        self.history.remove_text(window_id, removed_text, index, combine);
        return self.advance(1);
    }

    pub fn set_text(&mut self, window_id: usize, text: SharedString) -> usize {
        self.history.pop_until(self.history_index);
        self.history.remove_text(window_id, self.text.clone(), 0, false);
        self.history.insert_text(window_id, 0, text.clone(), false);
        self.text = text;
        return self.advance(2);
    }

    pub fn add_selection(&mut self, window_id: usize, index: usize, primary_index: usize, secondary_index: usize, offset: usize, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.add_selection(window_id, index, primary_index, secondary_index, offset, combine);
        return self.advance(1);
    }

    pub fn remove_selection(&mut self, window_id: usize, index: usize, primary_index: usize, secondary_index: usize, offset: usize, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.remove_selection(window_id, index, primary_index, secondary_index, offset, combine);
        return self.advance(1);
    }

    pub fn change_primary_index(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.change_primary_index(window_id, index, previous, new, combine);
        return self.advance(1);
    }

    pub fn change_secondary_index(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.change_secondary_index(window_id, index, previous, new, combine);
        return self.advance(1);
    }

    pub fn change_offset(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.change_offset(window_id, index, previous, new, combine);
        return self.advance(1);
    }

    pub fn change_selection_mode(&mut self, window_id: usize, previous: SelectionMode, new: SelectionMode, combine: bool) -> usize {
        self.history.pop_until(self.history_index);
        self.history.change_selection_mode(window_id, previous, new, combine);
        return self.advance(1);
    }

    pub fn set_text_without_save(&mut self, text: SharedString) {
        self.text = text;
    }

    pub fn get_text(&self) -> SharedString {
        return self.text.clone();
    }

    pub fn length(&self) -> usize {
        return self.text.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.text.is_empty();
    }

    pub fn character(&self, index: usize) -> Character {
        return self.text[index];
    }

    pub fn last_character(&self) -> Character {
        return self.text[self.last_buffer_index()];
    }

    pub fn get_history_index(&self) -> usize {
        return self.history_index;
    }

    pub fn get_action(&self, index: usize) -> BufferAction {
        return self.history.get(index);
    }

    fn do_buffer_action(&mut self, action: BufferAction) {
        match action {
            BufferAction::RemoveText(_window_id, text, index) => self.remove_text_raw(index, text.len()),
            BufferAction::InsertText(_window_id, text, index) => self.insert_text_raw(index, &text),
            invalid => panic!("buffer action {:?} may not be executed", invalid),
        }
    }

    pub fn undo(&mut self, language_manager: &mut LanguageManager, window_id: usize) {
        if self.history_index == 0 {
            return;
        }

        let mut force_retokenize = false;
        let mut history_index = self.history_index - 1;
        let mut action = self.history.get(history_index);

        while !action.is_text() && !action.is_selection(window_id) {
            if history_index == 0 {
                return;
            }

            history_index -= 1;
            action = self.history.get(history_index);
        }

        while history_index > 0 && !action.is_text() && !action.is_selection(window_id) {
            action = self.history.get(history_index);
        }

        while history_index > 0 {

            if action.is_text() {
                self.do_buffer_action(action.clone().invert());
                force_retokenize = true;
            }

            if !self.history.is_action_combined(history_index) {
                break;
            }

            history_index -= 1;
            action = self.history.get(history_index);
        }

        self.history_index = history_index;

        if force_retokenize {
            self.retokenize(language_manager);
        }
    }

    pub fn redo(&mut self, language_manager: &mut LanguageManager, window_id: usize) {
        let history_length = self.history.length();
        if self.history_index >= history_length {
            return;
        }

        let mut force_retokenize = false;
        let mut history_index = self.history_index;
        let mut action = self.history.get(history_index);

        while !action.is_text() && !action.is_selection(window_id) {
            history_index += 1;

            if history_index >= history_length {
                return;
            }

            action = self.history.get(history_index);
        }

        while history_index < history_length {

            if action.is_text() {
                self.do_buffer_action(action.clone());
                force_retokenize = true;
            }

            history_index += 1;

            if history_index >= history_length || !self.history.is_action_combined(history_index) {
                break;
            }

            action = self.history.get(history_index);
        }

        self.history_index = history_index;

        if force_retokenize {
            self.retokenize(language_manager);
        }
    }
}
