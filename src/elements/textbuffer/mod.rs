mod word;
mod info;
mod context;

use seamonkey::*;
use seamonkey::tokenize::Tokenizer;
//use parse::parse;

use std::cmp::{ min, max };

use sfml::graphics::RenderTexture;
use sfml::system::Vector2f;

use input::Action;
use themes::{ TextbufferTheme, StatusBarTheme };
use interface::InterfaceContext;
use selection::{ Selection, SelectionMode };
use filebuffer::{ Filebuffer, BufferAction };
use managers::LanguageManager;
use elements::{ Text, Field, Textfield };
use system::subtract_or_zero;

pub use self::word::Word;
pub use self::info::LineInfo;
pub use self::context::TextbufferContext;

macro_rules! handle_return {
    ($expression: expr) => ({
        $expression;
        return None;
    })
}

pub struct Textbuffer {
    selections: Vec<Selection>,
    mode: SelectionMode,
    adding_selection: bool,
    padding: Character,
    size: Vector2f,
    position: Vector2f,
    vertical_scroll: usize,
    horizontal_scroll: usize,
    history_index: usize,
    line_count: usize,
    window_id: usize,
}

impl Textbuffer {

    pub fn new(window_id: usize, size: Vector2f, position: Vector2f, padding: char) -> Self {
        Self {
            selections: vec![Selection::new(0, 0, 0)],
            mode: SelectionMode::Character,
            adding_selection: false,
            padding: Character::from_char(padding),
            size: size,
            position: position,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            history_index: 0,
            line_count: 1,
            window_id: window_id,
        }
    }

    pub fn set_text(&mut self, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text(self.window_id, text);
        self.reset(filebuffer);
        return filebuffer.retokenize(language_manager);
    }

    pub fn set_text_without_save(&mut self, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text_without_save(text);
        self.reset(filebuffer); // TODO: dont add change_selection_mode to undo queue
        return filebuffer.retokenize(language_manager);
    }

    pub fn get_selections(&self) -> Vec<Selection> {
        return self.selections.clone();
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer, size: Vector2f) {
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        self.line_count = (size.y / line_scaling) as usize;
        self.size = size;

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.position = position;
    }

    fn set_selection_mode(&mut self, filebuffer: &mut Filebuffer, mode: SelectionMode) {
        self.history_index = filebuffer.change_selection_mode(self.window_id, self.mode, mode, true);
        self.mode = mode;
    }

    pub fn reset(&mut self, filebuffer: &mut Filebuffer) { // MAKE
        self.vertical_scroll = 0;
        self.horizontal_scroll = 0;

        //self.drop_selections(filebuffer, textbuffer_context);
        //self.set_primary_index(filebuffer, 0, 0);
        //self.reset_selection(filebuffer, 0);

        self.selections = vec![Selection::new(0, 0, 0)];
        self.adding_selection = false;
        self.character_mode(filebuffer);
        self.history_index = filebuffer.get_history_index();
    }

    pub fn scroll_up(&mut self, textbuffer_context: &TextbufferContext) {
        match self.vertical_scroll >= textbuffer_context.scroll_size {
            true => self.vertical_scroll -= textbuffer_context.scroll_size,
            false => self.vertical_scroll = 0,
        }
    }

    pub fn scroll_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        self.vertical_scroll += textbuffer_context.scroll_size;
        self.check_bottom_scroll(textbuffer_context, filebuffer);
    }

    fn check_bottom_scroll(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        let last_line = self.line_number_from_index(filebuffer, filebuffer.last_buffer_index());
        let scroll_gap = textbuffer_context.selection_gap + 1;

        if last_line + scroll_gap < self.line_count {
            self.vertical_scroll = 0;
        } else if self.line_count > scroll_gap + (last_line - self.vertical_scroll) { // FIX breaks when scroll_gap > line_count (i think)
            self.vertical_scroll = scroll_gap + last_line - self.line_count;
        }
    }

    fn adjust_start_index(&self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer, disabled: bool, buffer_index: usize, range: usize) -> usize {
        let mut last_safe = buffer_index - range;

        if !disabled && textbuffer_context.start_at_symbol {
            for current_index in (buffer_index - range..buffer_index).rev() {
                if !filebuffer.character(current_index).is_whitespace() {
                    last_safe = current_index;
                }
            }
        }

        return last_safe;
    }

    fn check_selection_gaps(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {

        if !textbuffer_context.multiline {
            return;
        }

        let selection = self.selections.last().unwrap();
        let line_number = self.line_number_from_index(filebuffer, selection.primary_index);

        if textbuffer_context.selection_gap * 2 >= self.line_count {
            self.vertical_scroll = subtract_or_zero(line_number, self.line_count / 2);
        } else if line_number < self.vertical_scroll + textbuffer_context.selection_gap {
            match line_number > textbuffer_context.selection_gap {
                true => self.vertical_scroll = line_number - textbuffer_context.selection_gap,
                false => self.vertical_scroll = 0,
            }
        } else if line_number + textbuffer_context.selection_gap + 1 > self.vertical_scroll + self.line_count {
            self.vertical_scroll += line_number + textbuffer_context.selection_gap + 1 - self.vertical_scroll - self.line_count;
        }

        self.check_bottom_scroll(textbuffer_context, filebuffer);
    }

    fn add_selection_(&mut self, filebuffer: &mut Filebuffer, selection: Selection) {
        let index = self.selections.len();
        self.history_index = filebuffer.add_selection(self.window_id, index, selection.primary_index, selection.secondary_index, selection.offset, false);
        self.selections.push(selection);
    }

    fn remove_selection(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let secondary_index = self.selections[index].secondary_index;
        let offset = self.selections[index].offset;
        self.history_index = filebuffer.remove_selection(self.window_id, index, primary_index, secondary_index, offset, false);
        self.selections.remove(index);
    }

    fn set_primary_index(&mut self, filebuffer: &mut Filebuffer, index: usize, new_primary: usize) {
        let previous = self.selections[index].primary_index;
        if previous != new_primary {
            self.selections[index].primary_index = new_primary;
            self.history_index = filebuffer.change_primary_index(self.window_id, index, previous, new_primary, true);
        }
    }

    fn set_secondary_index(&mut self, filebuffer: &mut Filebuffer, index: usize, new_secondary: usize) {
        let previous = self.selections[index].secondary_index;
        if previous != new_secondary {
            self.selections[index].secondary_index = new_secondary;
            self.history_index = filebuffer.change_secondary_index(self.window_id, index, previous, new_secondary, true);
        }
    }

    fn move_selection_left(&mut self, filebuffer: &mut Filebuffer, index: usize) -> bool {
        if self.selections[index].primary_index > 0 {
            let new_primary = self.selections[index].primary_index - 1;
            self.set_primary_index(filebuffer, index, new_primary);
            return true;
        }
        return false;
    }

    fn move_selection_right(&mut self, filebuffer: &mut Filebuffer, index: usize) -> bool {
        if self.selections[index].primary_index < filebuffer.last_buffer_index() {
            let new_primary = self.selections[index].primary_index + 1;
            self.set_primary_index(filebuffer, index, new_primary);
            return true;
        }
        return false;
    }

    fn move_selection_down(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;

        for current_index in primary_index..filebuffer.last_buffer_index() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.line_length_from_index(filebuffer, current_index + 1);
                let distance_to_offset = min(line_length, self.selections[index].offset + 1);
                self.set_primary_index(filebuffer, index, current_index + distance_to_offset);
                return;
            }
        }

        if primary_index != filebuffer.last_buffer_index() {
            self.set_primary_index(filebuffer, index, filebuffer.last_buffer_index());
        }
    }

    fn move_selection_up(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;

        for current_index in (0..primary_index).rev() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.reverse_line_length_from_index(filebuffer, current_index) - 1;
                let distance_to_offset = line_length - min(line_length, self.selections[index].offset);
                self.set_primary_index(filebuffer, index, current_index - distance_to_offset);
                return;
            }
        }

        if primary_index != 0 {
            self.set_primary_index(filebuffer, index, 0);
        }
    }

    fn lower_word(&mut self, filebuffer: &mut Filebuffer, index: usize) -> Word {
        let primary_index = self.selections[index].primary_index;

        for current_index in primary_index..filebuffer.last_buffer_index() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.line_length_from_index(filebuffer, current_index + 1);
                let distance_to_offset = min(line_length, self.selections[index].offset + 1);
                return filebuffer.word_from_index(current_index + distance_to_offset);
            }
        }

        return filebuffer.last_word();
    }

    fn higher_word(&mut self, filebuffer: &mut Filebuffer, index: usize) -> Word {
        let primary_index = self.selections[index].primary_index;

        for current_index in (0..primary_index).rev() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.reverse_line_length_from_index(filebuffer, current_index) - 1;
                let distance_to_offset = line_length - min(line_length, self.selections[index].offset);
                return filebuffer.word_from_index(current_index - distance_to_offset);
            }
        }

        return filebuffer.first_word();
    }

    fn move_selection_to_end(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let distance_to_newline = self.line_length_from_index(filebuffer, primary_index);
        let new_primary = self.selections[index].primary_index + distance_to_newline - 1;
        self.set_primary_index(filebuffer, index, new_primary);
    }

    fn move_selection_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer, complete: bool, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let distance_to_newline = self.reverse_line_length_from_index(filebuffer, primary_index);
        let adjusted_index = self.adjust_start_index(textbuffer_context, filebuffer, complete, primary_index, distance_to_newline - 1);
        self.set_primary_index(filebuffer, index, adjusted_index);
    }

    fn move_secondary_to_end(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let distance_to_newline = self.line_length_from_index(filebuffer, secondary_index);
        let new_secondary = self.selections[index].secondary_index + distance_to_newline - 1;
        self.set_secondary_index(filebuffer, index, new_secondary);
    }

    fn move_secondary_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer, complete: bool, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let distance_to_newline = self.reverse_line_length_from_index(filebuffer, secondary_index);
        let adjusted_index = self.adjust_start_index(textbuffer_context, filebuffer, complete, secondary_index, distance_to_newline - 1);
        self.set_secondary_index(filebuffer, index, adjusted_index);
    }

    fn move_selection_to_end_of_word(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let word = filebuffer.word_from_index(primary_index);
        let new_primary = word.index + word.length - 1;
        self.set_primary_index(filebuffer, index, new_primary);
    }

    fn move_selection_to_start_of_word(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let word = filebuffer.word_from_index(primary_index);
        let new_primary = word.index;
        self.set_primary_index(filebuffer, index, new_primary);
    }

    fn move_secondary_to_end_of_word(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let word = filebuffer.word_from_index(secondary_index);
        let new_secondary = word.index + word.length - 1;
        self.set_secondary_index(filebuffer, index, new_secondary);
    }

    fn move_secondary_to_start_of_word(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let word = filebuffer.word_from_index(secondary_index);
        let new_secondary = word.index;
        self.set_secondary_index(filebuffer, index, new_secondary);
    }

    fn selection_smallest_index(&self, index: usize) -> usize {
        return min(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn selection_biggest_index(&self, index: usize) -> usize {
        return max(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn move_selection_to_first(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_primary_index(filebuffer, index, self.selection_smallest_index(index));
    }

    fn move_secondary_to_first(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_secondary_index(filebuffer, index, self.selection_smallest_index(index));
    }

    fn move_selection_to_last(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_primary_index(filebuffer, index, self.selection_biggest_index(index));
    }

    fn is_selection_extended(&self, index: usize) -> bool {
        return self.selections[index].primary_index != self.selections[index].secondary_index;
    }

    fn is_selection_inverted(&self, index: usize) -> bool {
        return self.selections[index].primary_index < self.selections[index].secondary_index;
    }

    fn set_offset(&mut self, filebuffer: &mut Filebuffer, index: usize, offset_index: usize) {
        let previous = self.selections[index].offset;
        let new_offset = self.offset_from_index(filebuffer, offset_index);
        if previous != new_offset {
            self.selections[index].offset = new_offset;
            filebuffer.change_offset(self.window_id, index, previous, new_offset, true);
        }
    }

    fn update_offset(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_offset(filebuffer, index, self.selections[index].primary_index);
    }

    fn update_offset_smallest(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_offset(filebuffer, index, self.selection_smallest_index(index));
    }

    fn is_selection_edge(&mut self, filebuffer: &mut Filebuffer, index: usize) -> bool {
        let smallest_index = self.selection_smallest_index(index);
        let biggest_index = self.selection_biggest_index(index);
        return smallest_index == 0 || biggest_index == filebuffer.last_buffer_index();
    }

    fn select_word(&mut self, filebuffer: &mut Filebuffer, index: usize, word: Word) {
        self.set_secondary_index(filebuffer, index, word.index);
        self.set_primary_index(filebuffer, index, word.index + word.length - 1);
    }

    fn select_to_word(&mut self, filebuffer: &mut Filebuffer, index: usize, word: Word) {
        self.set_primary_index(filebuffer, index, word.index);

        if self.is_selection_inverted(index) {
            self.move_secondary_to_end_of_word(filebuffer, index);
            self.move_selection_to_start_of_word(filebuffer, index);
        } else {
            self.move_secondary_to_start_of_word(filebuffer, index);
            self.move_selection_to_end_of_word(filebuffer, index);
        }
    }

    fn left_word(&self, filebuffer: &mut Filebuffer, index: usize) -> Word {
        return filebuffer.left_word(self.selections[index].primary_index);
    }

    fn right_word(&self, filebuffer: &mut Filebuffer, index: usize) -> Word {
        return filebuffer.right_word(self.selections[index].primary_index);
    }

    fn first_selected_word(&self, filebuffer: &Filebuffer, index: usize) -> Word {
        let first_index = self.selection_smallest_index(index);
        return filebuffer.word_from_index(first_index);
    }

    fn last_selected_word(&self, filebuffer: &Filebuffer, index: usize) -> Word {
        let last_index = self.selection_biggest_index(index);
        return filebuffer.word_from_index(last_index);
    }

    fn is_selection_multiword(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        return self.first_selected_word(filebuffer, index).index != self.last_selected_word(filebuffer, index).index;
    }

    fn reset_selection(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_secondary_index(filebuffer, index, self.selections[index].primary_index);
    }

    fn duplicate_selection(&mut self, filebuffer: &mut Filebuffer, index: usize) -> usize {
        let new_index = self.selections.len();
        self.add_selection_(filebuffer, self.selections.last().unwrap().clone());
        return new_index;
    }

    fn selection_length(&self, index: usize) -> usize {
        return self.selection_biggest_index(index) - self.selection_smallest_index(index) + 1;
    }

    fn is_selection_newline(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        let buffer_index = self.selections[index].primary_index;
        return filebuffer.character(buffer_index).is_newline();
    }

    fn is_last_selected_newline(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        let last_index = self.selection_biggest_index(index);
        return filebuffer.character(last_index).is_newline();
    }

    fn selection_exclude_last(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let new_last_index = self.selection_biggest_index(index) - 1;
        match self.is_selection_inverted(index) {
            true => self.set_secondary_index(filebuffer, index, new_last_index),
            false => self.set_primary_index(filebuffer, index, new_last_index),
        }
    }

    fn validate_text(&mut self, filebuffer: &mut Filebuffer) {
        if filebuffer.is_empty() || filebuffer.last_character() != self.padding {
            self.insert_text(filebuffer, filebuffer.length(), self.padding.to_string());
        }
    }

    fn insert_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, text: SharedString) {
        self.history_index = filebuffer.insert_text(self.window_id, buffer_index, text, true);
    }

    fn remove_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, length: usize) {
        self.history_index = filebuffer.remove_text(self.window_id, buffer_index, length, true);
        self.validate_text(filebuffer);
    }

    fn clip_selection(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let clipped = min(self.selections[index].primary_index, filebuffer.last_buffer_index());
        self.set_primary_index(filebuffer, index, clipped);
    }

    fn set_selection_length(&mut self, filebuffer: &mut Filebuffer, index: usize, length: usize) {
        let last_index = self.selection_smallest_index(index) + length - 1;

        match self.is_selection_inverted(index) {
            true => self.set_secondary_index(filebuffer, index, last_index),
            false => self.set_primary_index(filebuffer, index, last_index),
        }
    }

    fn delete_selected(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let buffer_index = self.selection_smallest_index(index);
        let selection_length = self.selection_length(index);
        self.remove_text(filebuffer, buffer_index, selection_length);
        self.move_selection_to_first(filebuffer, index);
        self.clip_selection(filebuffer, index);
        self.update_offset(filebuffer, index);
        self.reset_selection(filebuffer, index);
        self.unadvance_selections(filebuffer, index, selection_length);
    }

    fn delete_selected_primary(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let buffer_index = self.selections[index].primary_index;
        self.remove_text(filebuffer, buffer_index, 1);
        self.clip_selection(filebuffer, index);
        self.update_offset(filebuffer, index);
        self.reset_selection(filebuffer, index);
        self.unadvance_selections(filebuffer, index, 1);
    }

    fn is_selection_end_of_buffer(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        return self.selections[index].primary_index == filebuffer.last_buffer_index();
    }

    fn append(&mut self, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_last(filebuffer, index);

            if !self.is_selection_newline(filebuffer, index) {
                self.move_selection_right(filebuffer, index);
            }

            self.update_offset(filebuffer, index);
            self.reset_selection(filebuffer, index);
        }

        self.character_mode(filebuffer);
        //self.merge_overlapping_selections();
    }

    fn insert(&mut self, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_first(filebuffer, index);
            self.update_offset(filebuffer, index);
            self.reset_selection(filebuffer, index);
        }

        self.character_mode(filebuffer);
        //self.merge_overlapping_selections();
    }

    fn newline_up(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_first(filebuffer, index);
            self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
            self.update_offset(filebuffer, index);
            self.reset_selection(filebuffer, index);
        }

        self.character_mode(filebuffer);
        //self.merge_overlapping_selections();

        for index in self.selection_start()..self.selections.len() {
            self.insert_text(filebuffer, self.selections[index].primary_index, SharedString::from("\n"));
            self.advance_selections(filebuffer, index, 1);
        }

        filebuffer.retokenize(language_manager);
    }

    fn newline_down(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_last(filebuffer, index);
            self.move_selection_to_end(filebuffer, index);
        }

        self.character_mode(filebuffer);
        //self.merge_overlapping_selections();

        for index in self.selection_start()..self.selections.len() {
            let newline_index = self.selections[index].primary_index + 1;
            self.insert_text(filebuffer, newline_index, SharedString::from("\n"));
            self.advance_selections(filebuffer, index, 1);
            self.move_selection_right(filebuffer, index);
            self.update_offset(filebuffer, index);
            self.reset_selection(filebuffer, index);
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        filebuffer.retokenize(language_manager);
    }

    fn remove(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    if self.is_selection_extended(index) {
                        self.delete_selected(filebuffer, index);
                    } else if self.move_selection_left(filebuffer, index) {
                        self.delete_selected_primary(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.right_word(filebuffer, index);
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_first(filebuffer, index);
                    self.set_selection_length(filebuffer, index, word.length);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
        filebuffer.retokenize(language_manager);
    }

    fn delete(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    if self.is_selection_extended(index) {
                        self.delete_selected(filebuffer, index);
                    } else if !self.is_selection_end_of_buffer(filebuffer, index) {
                        self.delete_selected_primary(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.right_word(filebuffer, index);
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_first(filebuffer, index);
                    self.set_selection_length(filebuffer, index, word.length);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
        filebuffer.retokenize(language_manager);
    }

    fn delete_line(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    if self.is_selection_inverted(index) {
                        self.move_secondary_to_end(filebuffer, index);
                        self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
                    } else {
                        self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                        self.move_selection_to_end(filebuffer, index);
                    }

                    self.delete_selected(filebuffer, index);
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    if self.is_selection_inverted(index) {
                        self.move_secondary_to_end(filebuffer, index);
                        self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
                    } else {
                        self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                        self.move_selection_to_end(filebuffer, index);
                    }

                    let word = self.right_word(filebuffer, index);
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_first(filebuffer, index);
                    self.set_selection_length(filebuffer, index, word.length);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    self.delete_selected(filebuffer, index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
        filebuffer.retokenize(language_manager);
    }

    fn move_left(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_extended(index) {
                        true => self.move_selection_to_first(filebuffer, index),
                        false => { self.move_selection_left(filebuffer, index); },
                    }
                    self.update_offset(filebuffer, index);
                    self.reset_selection(filebuffer, index);
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = match self.is_selection_multiword(filebuffer, index) {
                        true => self.first_selected_word(filebuffer, index),
                        false => self.left_word(filebuffer, index),
                    };
                    self.select_word(filebuffer, index, word);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => return,
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn move_right(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_extended(index) {
                        true => self.move_selection_to_last(filebuffer, index),
                        false => { self.move_selection_right(filebuffer, index); },
                    }
                    self.update_offset(filebuffer, index);
                    self.reset_selection(filebuffer, index);
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = match self.is_selection_multiword(filebuffer, index) {
                        true => self.last_selected_word(filebuffer, index),
                        false => self.right_word(filebuffer, index),
                    };
                    self.select_word(filebuffer, index, word);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => return,
        }

        //self.check_selection_gaps(context);
        //self.merge_overlapping_selections();
    }

    fn move_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_first(filebuffer, index),
                        false => self.move_selection_up(filebuffer, index),
                    }
                    self.reset_selection(filebuffer, index);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = match self.is_selection_multiword(filebuffer, index) {
                        true => self.first_selected_word(filebuffer, index),
                        false => self.higher_word(filebuffer, index),
                    };
                    self.select_word(filebuffer, index, word);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset_smallest(filebuffer, index);
                    }
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_first(filebuffer, index),
                        false => self.move_selection_up(filebuffer, index),
                    }
                    self.reset_selection(filebuffer, index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn move_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_last(filebuffer, index),
                        false => self.move_selection_down(filebuffer, index),
                    }

                    self.reset_selection(filebuffer, index);
                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = match self.is_selection_multiword(filebuffer, index) {
                        true => self.last_selected_word(filebuffer, index),
                        false => self.lower_word(filebuffer, index),
                    };

                    self.select_word(filebuffer, index, word);
                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset_smallest(filebuffer, index);
                    }
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_last(filebuffer, index),
                        false => self.move_selection_down(filebuffer, index),
                    }
                    self.reset_selection(filebuffer, index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn extend_left(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_left(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.left_word(filebuffer, index);
                    self.select_to_word(filebuffer, index, word);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => return,
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn extend_right(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_right(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.right_word(filebuffer, index);
                    self.select_to_word(filebuffer, index, word);
                    self.update_offset_smallest(filebuffer, index);
                }
            },

            SelectionMode::Line => return,
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn extend_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_up(filebuffer, index);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.higher_word(filebuffer, index);
                    self.select_to_word(filebuffer, index, word);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset_smallest(filebuffer, index);
                    }
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_up(filebuffer, index);

                    if self.is_selection_inverted(index) {
                        self.move_secondary_to_end(filebuffer, index);
                        self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
                    } else {
                        self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                        self.move_selection_to_end(filebuffer, index);
                    }

                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn extend_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_down(filebuffer, index);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset(filebuffer, index);
                    }
                }
            },

            SelectionMode::Word => {
                for index in self.selection_start()..self.selections.len() {
                    let word = self.lower_word(filebuffer, index);
                    self.select_to_word(filebuffer, index, word);

                    if self.is_selection_edge(filebuffer, index) {
                        self.update_offset_smallest(filebuffer, index);
                    }
                }
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_down(filebuffer, index);

                    if self.is_selection_inverted(index) {
                        self.move_secondary_to_end(filebuffer, index);
                        self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
                    } else {
                        self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                        self.move_selection_to_end(filebuffer, index);
                    }

                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
        //self.merge_overlapping_selections();
    }

    fn move_to_end(&mut self, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                    self.reset_selection(filebuffer, index);
                }
            },

            SelectionMode::Word => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn move_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_to_start(textbuffer_context, filebuffer, false, index);
                    self.update_offset(filebuffer, index);
                    self.reset_selection(filebuffer, index);
                }
            },

            SelectionMode::Word => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn extend_end(&mut self, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },

            SelectionMode::Word => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn extend_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    let select_entire_line = self.is_selection_multiline(filebuffer, index);
                    self.move_selection_to_start(textbuffer_context, filebuffer, select_entire_line, index);
                    self.update_offset(filebuffer, index);
                }
            },

            SelectionMode::Word => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn selection_start(&self) -> usize {
        match self.adding_selection {
            true => return self.selections.len() - 1,
            false => return 0,
        }
    }

    fn index_from_line(&self, filebuffer: &Filebuffer, line: usize) -> usize {
        let mut line_count = 0;

        for index in 0..filebuffer.length() {
            if line_count == line {
                return index;
            }

            if filebuffer.character(index).is_newline() {
                line_count += 1;
            }
        }

        return filebuffer.last_buffer_index();
    }

    fn line_number_from_index(&self, filebuffer: &Filebuffer, index: usize) -> usize {
        let mut line_count = 0;

        for current_index in 0..filebuffer.length() {
            if current_index == index {
                return line_count;
            }

            if filebuffer.character(current_index).is_newline() {
                line_count += 1;
            }
        }

        return line_count;
    }

    fn offset_from_index(&self, filebuffer: &Filebuffer, index: usize) -> usize {
        let mut left_offset = 0;

        for current_index in (0..index).rev() {
            match filebuffer.character(current_index).is_newline() {
                true => return left_offset,
                false => left_offset += 1,
            }
        }

        return left_offset;
    }

    fn line_length_from_index(&self, filebuffer: &Filebuffer, index: usize) -> usize {
        let mut length = 1;

        for current_index in index..filebuffer.last_buffer_index() {
            if filebuffer.character(current_index).is_newline() {
                return length;
            }
            length += 1;
        }

        return length;
    }

    fn reverse_line_length_from_index(&self, filebuffer: &Filebuffer, index: usize) -> usize {
        let mut length = 1;

        for current_index in (0..index).rev() {
            if filebuffer.character(current_index).is_newline() {
                return length;
            }
            length += 1;
        }

        return length;
    }

    fn is_selection_multiline(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        for current_index in self.selection_smallest_index(index)..self.selection_biggest_index(index) {
            if filebuffer.character(current_index).is_newline() {
                return true;
            }
        }
        return false;
    }

    fn character_mode(&mut self, filebuffer: &mut Filebuffer) {
        if !self.mode.is_character() {
            self.set_selection_mode(filebuffer, SelectionMode::Character);
        }
    }

    fn word_mode(&mut self, filebuffer: &mut Filebuffer) {
        if !self.mode.is_word() {
            self.set_selection_mode(filebuffer, SelectionMode::Word);

            for index in 0..self.selections.len() {
                if self.is_selection_inverted(index) {
                    self.move_secondary_to_end_of_word(filebuffer, index);
                    self.move_selection_to_start_of_word(filebuffer, index);
                } else {
                    self.move_secondary_to_start_of_word(filebuffer, index);
                    self.move_selection_to_end_of_word(filebuffer, index);
                }
            }

            //self.merge_overlapping_selections();
        }
    }

    fn line_mode(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        if !self.mode.is_line() {
            self.set_selection_mode(filebuffer, SelectionMode::Line);

            for index in 0..self.selections.len() {
                if self.is_selection_inverted(index) {
                    self.move_secondary_to_end(filebuffer, index);
                    self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
                } else {
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                }
            }

            //self.merge_overlapping_selections();
        }
    }

    fn add_selection(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                let buffer_index = self.selection_biggest_index(self.selections.len() - 1) + 1;
                let offset = self.offset_from_index(filebuffer, buffer_index);
                let new_selection = Selection::new(buffer_index, buffer_index, offset);
                self.add_selection_(filebuffer, new_selection);
                self.adding_selection = true;
            },

            SelectionMode::Word => {
            },

            SelectionMode::Line => {
                let buffer_index = self.selection_biggest_index(self.selections.len() - 1) + 1;
                let offset = self.offset_from_index(filebuffer, buffer_index);
                let new_selection = Selection::new(buffer_index, buffer_index, offset);
                self.add_selection_(filebuffer, new_selection);
                self.adding_selection = true;

                self.reset_selection(filebuffer, self.selections.len() - 1);
                self.move_selection_to_end(filebuffer, self.selections.len() - 1);
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn index_has_selection(&self, primary_index: usize, secondary_index: usize) -> bool {
        for index in 0..self.selections.len() {
            if primary_index >= self.selection_smallest_index(index) && secondary_index <= self.selection_biggest_index(index) {
                return true;
            }
        }
        return false;
    }

    fn sort_selection_matches(&self, index: usize, matches: &mut Vec<usize>) {
        let selection_start = self.selection_smallest_index(index);

        for _ in 0..matches.len() {
            if matches[0] == selection_start {
                // more optimization because the selected match can not be selected again
                matches.remove(0);
                return;
            }

            let match_buffer = matches.remove(0);
            matches.push(match_buffer);
        }
    }

    fn select_next(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {

                    let selection_length = self.selection_length(index);
                    let selection_buffer = self.get_selected_text(filebuffer, index);
                    let mut selection_matches = filebuffer.get_text().position(&selection_buffer);

                    self.sort_selection_matches(index, &mut selection_matches);
                    let primary_index = selection_matches[0];
                    let secondary_index = primary_index + selection_length - 1;

                    if !self.index_has_selection(primary_index, secondary_index) {
                        let offset = self.offset_from_index(filebuffer, primary_index);
                        let selection = Selection::new(primary_index, secondary_index, offset);
                        self.add_selection_(filebuffer, selection);
                    }
                }
            },

            SelectionMode::Word => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn duplicate_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                let selection_count = self.selections.len();
                for index in self.selection_start()..selection_count {
                    let new_index = self.duplicate_selection(filebuffer, index);
                    self.move_selection_up(filebuffer, new_index);
                    self.reset_selection(filebuffer, new_index);
                }
            },

            SelectionMode::Word => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn duplicate_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                let selection_count = self.selections.len();
                for index in self.selection_start()..selection_count {
                    let new_index = self.duplicate_selection(filebuffer, index);
                    self.move_selection_down(filebuffer, new_index);
                    self.reset_selection(filebuffer, new_index);
                }
            },

            SelectionMode::Word => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn do_buffer_action(&mut self, action: BufferAction) {
        match action {
            BufferAction::AddSelection(_window_id, index, primary_index, secondary_index, offset) => {
                let selection = Selection::new(primary_index, secondary_index, offset);
                match index == self.selections.len() {
                    true => self.selections.push(selection),
                    false => self.selections.insert(index, selection),
                }
            },
            BufferAction::RemoveSelection(_window_id, index, ..) => { self.selections.remove(index); },
            BufferAction::ChangePrimaryIndex(_window_id, index, _previous, new) => self.selections[index].primary_index = new,
            BufferAction::ChangeSecondaryIndex(_window_id, index, _previous, new) => self.selections[index].secondary_index = new,
            BufferAction::ChangeOffset(_window_id, index, _previous, new) => self.selections[index].offset = new,
            BufferAction::ChangeSelectionMode(_window_id, _previous, new) => self.mode = new,
            invalid => panic!("buffer action {:?} may not be executed", invalid),
        }
    }

    fn adjust_selections(&mut self, action: BufferAction) {
        match action {
            BufferAction::RemoveText(_window_id, text, index) => {
                let length = text.len();

                // additional logic for selections being deleted completely

                for selection in &mut self.selections {
                    if selection.primary_index >= index {
                        selection.primary_index -= length;
                    }
                    if selection.secondary_index >= index {
                        selection.secondary_index -= length;
                    }
                }
            },
            BufferAction::InsertText(_window_id, text, index) => {
                let length = text.len();

                // additional logic for selections being deleted completely

                for selection in &mut self.selections {
                    if selection.primary_index >= index {
                        selection.primary_index += length;
                    }
                    if selection.secondary_index >= index {
                        selection.secondary_index += length;
                    }
                }
            },
            invalid => panic!("buffer action {:?} may not be adjusted", invalid),
        }
    }

    pub fn history_catch_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) -> bool {
        let history_index = filebuffer.get_history_index();
        let force_rerender = self.history_index != history_index;

        while self.history_index > history_index {
            self.history_index -= 1;
            let action = filebuffer.get_action(self.history_index).invert();

            if action.is_selection(self.window_id) {
                self.do_buffer_action(action);
            } else if action.is_other_text(self.window_id) {
                self.adjust_selections(action);
            }
        }

        while self.history_index < history_index {
            let action = filebuffer.get_action(self.history_index);
            self.history_index += 1;

            if action.is_selection(self.window_id) {
                self.do_buffer_action(action);
            } else if action.is_other_text(self.window_id) {
                self.adjust_selections(action);
            }
        }

        if force_rerender {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }

        return force_rerender;
    }

    fn undo(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        filebuffer.undo(language_manager, self.window_id);
        if self.history_catch_up(textbuffer_context, filebuffer) {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }
    }

    fn redo(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        filebuffer.redo(language_manager, self.window_id);
        if self.history_catch_up(textbuffer_context, filebuffer) {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }
    }

    pub fn select_last_character(&mut self, filebuffer: &mut Filebuffer) {
        for _index in 0..self.selections.len() - 1 {
            self.remove_selection(filebuffer, 1);
        }

        self.adding_selection = false;
        self.character_mode(filebuffer);
        self.move_selection_to_end(filebuffer, 0);
        self.update_offset(filebuffer, 0);
        self.reset_selection(filebuffer, 0);
    }

    pub fn jump_to_index(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer, jump_index: usize) {
        for _index in 0..self.selections.len() - 1 {
            self.remove_selection(filebuffer, 1);
        }

        self.adding_selection = false;
        self.character_mode(filebuffer);
        self.set_primary_index(filebuffer, 0, jump_index);
        self.update_offset(filebuffer, 0);
        self.reset_selection(filebuffer, 0);
        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    pub fn handle_action(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, action: Action) -> Option<Action> {
        match action {

            Action::CharacterMode => handle_return!(self.character_mode(filebuffer)),

            Action::WordMode => handle_return!(self.word_mode(filebuffer)),

            Action::LineMode => handle_return!(self.line_mode(textbuffer_context, filebuffer)),

            Action::Down => handle_return!(self.move_down(textbuffer_context, filebuffer)),

            Action::Up => handle_return!(self.move_up(textbuffer_context, filebuffer)),

            Action::Left => handle_return!(self.move_left(textbuffer_context, filebuffer)),

            Action::Right => handle_return!(self.move_right(textbuffer_context, filebuffer)),

            Action::ExtendDown => handle_return!(self.extend_down(textbuffer_context, filebuffer)),

            Action::ExtendUp => handle_return!(self.extend_up(textbuffer_context, filebuffer)),

            Action::ExtendLeft => handle_return!(self.extend_left(textbuffer_context, filebuffer)),

            Action::ExtendRight => handle_return!(self.extend_right(textbuffer_context, filebuffer)),

            Action::Start => handle_return!(self.move_to_start(textbuffer_context, filebuffer)),

            Action::End => handle_return!(self.move_to_end(filebuffer)),

            Action::ExtendStart => handle_return!(self.extend_start(textbuffer_context, filebuffer)),

            Action::ExtendEnd => handle_return!(self.extend_end(filebuffer)),

            Action::DuplicateUp => handle_return!(self.duplicate_up(textbuffer_context, filebuffer)),

            Action::DuplicateDown => handle_return!(self.duplicate_down(textbuffer_context, filebuffer)),

            Action::Append => handle_return!(self.append(filebuffer)),

            Action::Insert => handle_return!(self.insert(filebuffer)),

            Action::NewlineUp => handle_return!(self.newline_up(textbuffer_context, language_manager, filebuffer)),

            Action::NewlineDown => handle_return!(self.newline_down(textbuffer_context, language_manager, filebuffer)),

            Action::AddSelection => handle_return!(self.add_selection(textbuffer_context, filebuffer)),

            Action::SelectNext => handle_return!(self.select_next(textbuffer_context, filebuffer)),

            Action::Remove => handle_return!(self.remove(textbuffer_context, language_manager, filebuffer)),

            Action::Delete => handle_return!(self.delete(textbuffer_context, language_manager, filebuffer)),

            Action::DeleteLine => handle_return!(self.delete_line(textbuffer_context, language_manager, filebuffer)),

            Action::Rotate => handle_return!(self.rotate_selections(language_manager, filebuffer)),

            Action::Undo => handle_return!(self.undo(textbuffer_context, language_manager, filebuffer)),

            Action::Redo => handle_return!(self.redo(textbuffer_context, language_manager, filebuffer)),

            Action::Abort => {
                if self.selections.len() > 1 {
                    self.drop_selections(filebuffer, textbuffer_context);
                    return None;
                }
                return Some(Action::Abort);
            },

            Action::Confirm => {
                if self.adding_selection {
                    self.adding_selection = false;
                    return None;
                }
                return Some(Action::Confirm);
            },

            unhandled => return Some(unhandled),
        }
    }

    fn unadvance_selections(&mut self, filebuffer: &Filebuffer, reference_index: usize, offset: usize) {
        let base_index = self.selections[reference_index].primary_index;

        for index in 0..self.selections.len() {
            if self.selections[index].primary_index > base_index {
                self.selections[index].primary_index -= offset; // ADD BOUNDS!!!!!!!!!!
                self.selections[index].secondary_index -= offset;
            }
        }
    }

    fn advance_selections(&mut self, filebuffer: &Filebuffer, reference_index: usize, offset: usize) {
        let base_index = self.selections[reference_index].primary_index;

        for index in 0..self.selections.len() {
            if self.selections[index].primary_index > base_index {
                self.selections[index].primary_index += offset; // ADD BOUNDS!!!!!!!!!!
                self.selections[index].secondary_index += offset;
            }
        }
    }

    fn get_selected_text(&self, filebuffer: &Filebuffer, index: usize) -> SharedString {
        let start = self.selection_smallest_index(index);
        let end = self.selection_biggest_index(index);
        return filebuffer.get_text().slice(start, end);
    }

    fn replace_selected_text(&mut self, filebuffer: &mut Filebuffer, index: usize, new_text: SharedString) {
        let current_length = self.selection_length(index);
        let new_length = new_text.len();

        let current_index = self.selection_smallest_index(index);
        let current_text = filebuffer.get_text().slice(current_index, current_index + current_length - 1);

        if current_text == new_text {
            return;
        }

        self.remove_text(filebuffer, current_index, current_length);
        self.insert_text(filebuffer, current_index, new_text);

        if current_length == new_length {
            return;
        }

        match current_length > new_length {
            true => self.unadvance_selections(filebuffer, index, current_length - new_length),
            false => self.advance_selections(filebuffer, index, new_length - current_length),
        }
    }

    fn rotate_selections(&mut self, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        if self.selections.len() > 1 {
            let mut buffer = self.get_selected_text(filebuffer, self.selections.len() - 1);

            for index in 0..self.selections.len() {
                let new_text = self.get_selected_text(filebuffer, index);
                let new_length = buffer.len();

                self.replace_selected_text(filebuffer, index, buffer);
                self.set_selection_length(filebuffer, index, new_length);
                self.update_offset(filebuffer, index);
                buffer = new_text;
            }

            self.validate_text(filebuffer);
            filebuffer.retokenize(language_manager);
        }
    }

    fn clear_selections(&mut self) {
        self.selections.clear();
        self.adding_selection = false;
    }

    fn drop_selections(&mut self, filebuffer: &mut Filebuffer, textbuffer_context: &TextbufferContext) {
        for _index in 0..self.selections.len() - 1 {
            self.remove_selection(filebuffer, 1);
        }

        // reset that selection
        self.adding_selection = false;
        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn set_selections_from_string(&mut self, filebuffer: &mut Filebuffer, string: &SharedString) {

        self.clear_selections();

        let positions = filebuffer.get_text().position(string);
        let length = string.len();

        for index in positions {
            let offset = self.offset_from_index(filebuffer, index);
            let selection = Selection::new(index, index + length, offset);
            self.selections.push(selection);
        }
    }

    pub fn add_character(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, character: Character) {

        for index in 0..self.selections.len() {
            if self.is_selection_extended(index) {

                if textbuffer_context.preserve_lines && self.is_last_selected_newline(filebuffer, index) {
                    self.selection_exclude_last(filebuffer, index);
                }

                self.replace_selected_text(filebuffer, index, character.to_string());
                self.move_selection_to_first(filebuffer, index);
                self.move_selection_right(filebuffer, index);
                self.update_offset(filebuffer, index);
                self.reset_selection(filebuffer, index);
            } else {
                let buffer_index = self.selections[index].primary_index;
                self.insert_text(filebuffer, buffer_index, character.to_string());
                self.move_selection_right(filebuffer, index);
                self.update_offset(filebuffer, index);
                self.reset_selection(filebuffer, index);
                self.advance_selections(filebuffer, index, 1);
            }
        }

        if character.is_newline() {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }

        self.adding_selection = false;
        self.character_mode(filebuffer);
        filebuffer.retokenize(language_manager);
    }

    fn render_text(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer, line_info: &Vec<LineInfo>) {

        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.line_number_width as f32 * character_scaling + theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };

        let mut top_offset = theme.offset.y * interface_context.font_size as f32;
        let mut left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
        let mut word_index = 0;

        let line_number_height = line_scaling - theme.line_number_gap * 2.0 * line_scaling;
        let line_number_size = Vector2f::new(theme.line_number_width * character_scaling, line_number_height);
        let character_size = Vector2f::new(character_scaling, line_scaling);

        let mut text_theme = match textbuffer_context.highlighting {
            true => filebuffer.word_theme(&theme, word_index),
            false => &theme.text_theme,
        };

        for line in line_info {

            if textbuffer_context.line_numbers {
                let position = self.position + Vector2f::new(theme.line_number_offset * interface_context.font_size as f32, top_offset + theme.line_number_gap * line_scaling);
                let theme = match line.highlighted {
                    true => &theme.highlighted_line_number_theme,
                    false => &theme.line_number_theme,
                };

                Textfield::render(framebuffer, interface_context, theme, &format_shared!("{}", line.number), line_number_size, position, line_number_size.y);
            }

            for index in line.index..filebuffer.length() {

                if filebuffer.character(index).is_newline() || left_offset > self.size.x {
                    left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
                    top_offset += line_scaling;
                    break;
                }

                while index >= filebuffer.word_last_index(word_index) {
                    word_index += 1;
                    if textbuffer_context.highlighting {
                        text_theme = filebuffer.word_theme(&theme, word_index);
                    }
                }

                let character_position = self.position + Vector2f::new(left_offset, top_offset);
                Text::render(framebuffer, interface_context, text_theme, &filebuffer.character(index).to_string(), character_size, character_position);
                left_offset += character_scaling;
            }
        }
    }

    fn render_selection_lines(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer, line_info: &Vec<LineInfo>) {

        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.line_number_width as f32 * character_scaling + theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };
        let left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
        let mut top_offset = theme.offset.y * interface_context.font_size as f32;
        let line_size = Vector2f::new(self.size.x, line_scaling);

        for line in line_info {
            if line.highlighted {
                let position = self.position + Vector2f::new(left_offset, top_offset);
                Field::render(framebuffer, interface_context, &theme.selection_line_theme, line_size, position, line_scaling);
            }
            top_offset += line_scaling;
        }
    }

    fn render_selections(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer) {

        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let scroll_offset = self.vertical_scroll as f32 * line_scaling + theme.offset.y * interface_context.font_size as f32;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.line_number_width as f32 * character_scaling + theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };

        for index in 0..self.selections.len() {

            let start_index = self.selection_smallest_index(index);
            let mut current_line = self.line_number_from_index(filebuffer, start_index);
            let mut top_offset = current_line as f32 * line_scaling + theme.offset.y * interface_context.font_size as f32;
            let mut left_offset = self.offset_from_index(filebuffer, start_index) as f32 * character_scaling + line_number_offset + theme.offset.x * interface_context.font_size as f32;
            let selection_length = self.selection_length(index);
            let selection_size = Vector2f::new(character_scaling, line_scaling);

            for offset in 0..selection_length {
                if current_line >= self.vertical_scroll {

                    let selection_theme = match self.adding_selection && index == self.selections.len() - 1 {
                        true => &theme.new_selection_theme,
                        false => &theme.selection_theme,
                    };

                    let selection_theme = if selection_length == 1 {
                        &selection_theme.single_selection_theme
                    } else if offset == 0 {
                        &selection_theme.first_selection_theme
                    } else if offset == selection_length - 1 {
                        &selection_theme.last_selection_theme
                    } else {
                        &selection_theme.center_selection_theme
                    };

                    let character = match start_index + offset < filebuffer.length() {
                        true => filebuffer.character(start_index + offset).to_string(),
                        false => SharedString::from(" "),
                    };

                    let position = self.position + Vector2f::new(left_offset, top_offset - scroll_offset + theme.offset.y * interface_context.font_size as f32);
                    Textfield::render(framebuffer, interface_context, selection_theme, &character, selection_size, position, character_scaling);
                } else if top_offset - scroll_offset > self.size.y {
                    break;
                }

                if filebuffer.character(start_index + offset).is_newline() {
                    left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
                    top_offset += line_scaling;
                    current_line += 1;
                } else {
                    left_offset += character_scaling;
                }
            }
        }
    }

    fn render_status_bar(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &StatusBarTheme, filebuffer: &Filebuffer) {

        //let mut status_bar_pieces = Vec::new();
        //status_bar_pieces.push(Piece::new(self.mode.name(), &theme.mode_theme));
        //status_bar_pieces.push(Piece::new(word.display_name(), &theme.word_theme));
        //StatusBar::render(status_bar_pieces);

        let mut status_bar_content = SharedString::new();

        let primary_index = self.selections[self.selections.len() - 1].primary_index;
        let line_number = self.line_number_from_index(filebuffer, primary_index) + 1;
        let character = self.offset_from_index(filebuffer, primary_index) + 1;
        let length = self.selection_biggest_index(self.selections.len() - 1) - self.selection_smallest_index(self.selections.len() - 1) + 1;

        let word = filebuffer.word_from_index(primary_index);

        if let Some(display_name) = word.display_name() {
            status_bar_content.push_str(&format_shared!("{}   ", display_name));
        }

        if let TokenType::Invalid(error) = word.token_type {
            status_bar_content.push_str(&format_shared!("{}   ", error.display(&None, &map!())));
        }

        status_bar_content.push_str(&format_shared!("{}:{}:{}   ", line_number, character, length));
        status_bar_content.push_str(&format_shared!("{}", self.mode.name()));

        let status_bar_height = theme.height * interface_context.font_size as f32;
        let size = Vector2f::new(self.size.x, status_bar_height);
        let position = Vector2f::new(0.0, self.size.y - status_bar_height);
        Textfield::render(framebuffer, interface_context, &theme.textfield_theme, &status_bar_content, size, position, interface_context.font_size as f32);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer, scaler: f32, focused: bool) {
        Field::render(framebuffer, interface_context, &theme.background_theme, self.size, self.position, scaler);

        let line_info = self.line_info(textbuffer_context, filebuffer);

        if textbuffer_context.selection_lines && (focused || textbuffer_context.unfocused_selections) {
            self.render_selection_lines(framebuffer, interface_context, textbuffer_context, theme, filebuffer, &line_info);
        }

        self.render_text(framebuffer, interface_context, textbuffer_context, theme, filebuffer, &line_info);

        if focused || textbuffer_context.unfocused_selections {
            self.render_selections(framebuffer, interface_context, textbuffer_context, theme, filebuffer);
        }

        if textbuffer_context.status_bar {
            self.render_status_bar(framebuffer, interface_context, &theme.status_bar_theme, filebuffer);
        }
    }

    pub fn line_info(&self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) -> Vec<LineInfo> {

        let mut line_info = Vec::new();
        let mut line_number = self.vertical_scroll;
        let index = self.index_from_line(filebuffer, line_number);
        let mut line_start = index;
        let mut line_length = 0;

        for index in index..filebuffer.length() {

            if line_number > self.vertical_scroll + self.line_count {
                break;
            }

            if filebuffer.character(index).is_newline() || index == filebuffer.last_buffer_index() {
                line_number += 1;
                line_info.push(LineInfo::new(line_number, line_start, line_length));
                line_start = index + 1;
                line_length = 0;
                continue;
            }

            line_length += 1;
        }

        for index in 0..self.selections.len() {
            let smallest_index = self.selection_smallest_index(index);
            let biggest_index = self.selection_biggest_index(index);

            for line in &mut line_info {
                if biggest_index >= line.index && smallest_index <= line.index + line.length {
                    line.highlighted = true;
                }
            }
        }

        if textbuffer_context.relative_line_numbers {
            let index = self.selections.len() - 1;

            let smallest_index = self.selection_smallest_index(index);
            let biggest_index = self.selection_biggest_index(index);

            let first_line = self.line_number_from_index(filebuffer, smallest_index) + 1;
            let last_line = self.line_number_from_index(filebuffer, biggest_index) + 1;

            for line in &mut line_info {
                if !line.highlighted {
                    if line.number < first_line {
                        line.number = first_line - line.number;
                    } else {
                        line.number = line.number - last_line;
                    }
                }
            }
        }

        return line_info;
    }
}
