use seamonkey::*;

use system::History;
use system::BufferAction;

#[derive(Clone)]
pub struct Filebuffer {
    text: SharedString,
    history: History,
    history_index: usize,
}

impl Filebuffer {

    pub fn new(text: SharedString) -> Self {
        return Self {
            text: text,
            history: History::new(),
            history_index: 0,
        }
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

    pub fn insert_text(&mut self, index: usize, text: SharedString) -> usize {
        self.insert_text_raw(index, &text);
        self.history.pop_until(self.history_index);

        if self.history.insert_text(index, text) {
            self.history_index += 1;
        }

        return self.history_index;
    }

    pub fn remove_text(&mut self, index: usize, length: usize) -> usize {
        let removed_text = self.text.slice(index, index + length - 1);
        self.remove_text_raw(index, length);
        self.history.pop_until(self.history_index);

        if self.history.remove_text(removed_text, index) {
            self.history_index += 1;
        }

        return self.history_index;
    }

    pub fn set_text(&mut self, text: SharedString) -> usize {
        self.history.pop_until(self.history_index);

        if self.history.remove_text(self.text.clone(), 0) {
            self.history_index += 1;
        }

        if self.history.insert_text(0, text.clone()) {
            self.history_index += 1;
        }

        self.text = text;
        return self.history_index;
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

    fn do_buffer_action(&mut self, action: BufferAction) {
        match action {
            BufferAction::RemoveText(text, index) => self.remove_text_raw(index, text.len()),
            BufferAction::InsertText(text, index) => self.insert_text_raw(index, &text),
        }
    }

    pub fn undo(&mut self) -> usize {
        if self.history_index > 0 {
            let action_buffer = self.history.get(self.history_index - 1);
            for index in (0..action_buffer.len()).rev() {
                self.do_buffer_action(action_buffer[index].invert());
            }
            self.history_index -= 1;
        }
        return self.history_index;
    }

    pub fn redo(&mut self) -> usize {
        if self.history_index < self.history.length() {
            for action in self.history.get(self.history_index) {
                self.do_buffer_action(action);
            }
            self.history_index += 1;
        }
        return self.history_index;
    }
}
