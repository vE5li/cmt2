mod token;
mod selection;
mod context;
mod theme;

use seamonkey::*;
use seamonkey::tokenize::Tokenizer;
//use parse::parse;

use std::cmp::{ min, max };

use sfml::graphics::RenderTexture;
use sfml::system::Vector2f;

use elements::*;
use interface::InterfaceContext;
use dialogues::*;
use input::Action;
use system::{ Filebuffer, BufferAction, LanguageManager };

pub use self::token::Token;

pub use self::theme::TextbufferTheme;
pub use self::context::TextbufferContext;
pub use self::selection::*;

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

    pub fn new(window_id: usize, size: Vector2f, position: Vector2f, padding: char, selection_lines: bool, status_bar: bool, multiline: bool) -> Self {
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
            line_count: 0,
            window_id: window_id,
        }
    }

    pub fn set_text(&mut self, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text(text);
        self.reset(filebuffer);
        return filebuffer.retokenize(language_manager);
    }

    pub fn set_text_without_save(&mut self, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text_without_save(text);
        self.reset(filebuffer); // TODO: dont add change_selection_mode to undo queue
        return filebuffer.retokenize(language_manager);
    }

    pub fn resize(&mut self, interface_context: &InterfaceContext, size: Vector2f) {
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        self.line_count = (size.y / line_scaling) as usize;
        self.size = size;
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.position = position;
    }

    fn set_selection_mode(&mut self, filebuffer: &mut Filebuffer, mode: SelectionMode) {
        filebuffer.change_selection_mode(self.window_id, self.mode, mode, true);
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
    }

    pub fn scroll_up(&mut self, textbuffer_context: &TextbufferContext) {
        match self.vertical_scroll >= textbuffer_context.scroll_size {
            true => self.vertical_scroll -= textbuffer_context.scroll_size,
            false => self.vertical_scroll = 0,
        }
    }

    pub fn scroll_down(&mut self, textbuffer_context: &TextbufferContext) {
        //if self.vertical_scroll < self.last {
            self.vertical_scroll += textbuffer_context.scroll_size;
        //}
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

    fn check_selection_gaps(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {

        if !textbuffer_context.multiline {
            return;
        }

        let selection = self.selections.last().unwrap();
        let line_number = self.line_number_from_index(filebuffer, selection.primary_index);

        if line_number < self.vertical_scroll + textbuffer_context.selection_gap {
            match line_number > textbuffer_context.selection_gap {
                true => self.vertical_scroll = line_number - textbuffer_context.selection_gap,
                false => self.vertical_scroll = 0,
            }
        } else if line_number + textbuffer_context.selection_gap > self.vertical_scroll + self.line_count {
            self.vertical_scroll += line_number + textbuffer_context.selection_gap - self.vertical_scroll - self.line_count;
        }
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

    fn selection_smallest_index(&self, index: usize) -> usize {
        return min(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn selection_biggest_index(&self, index: usize) -> usize {
        return max(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn move_selection_to_first(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        self.set_primary_index(filebuffer, index, self.selection_smallest_index(index));
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

    fn update_offset(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let previous = self.selections[index].offset;
        let new_offset = self.offset_from_index(filebuffer, self.selections[index].primary_index);
        if previous != new_offset {
            self.selections[index].offset = new_offset;
            filebuffer.change_offset(self.window_id, index, previous, new_offset, true);
        }
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
            self.insert_text(filebuffer, filebuffer.last_buffer_index(), self.padding.to_string());
        }
    }

    fn insert_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, text: SharedString) {
        self.history_index = filebuffer.insert_text(buffer_index, text, true);
    }

    fn remove_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, length: usize) {
        self.history_index = filebuffer.remove_text(buffer_index, length, true);
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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
                }
            },

            SelectionMode::Token => {
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
                }
            },

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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
                }
            },

            SelectionMode::Token => {
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
                }
            },

            SelectionMode::Token => {
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

            SelectionMode::Token => {

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

            SelectionMode::Token => {

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

            SelectionMode::Token => {

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

            SelectionMode::Token => {

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

    //fn line_count(&self, filebuffer: &Filebuffer) -> usize {
    //    return self.line_number_from_index(filebuffer.last_buffer_index());
    //}

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

    fn token_from_index(&self, filebuffer: &mut Filebuffer, index: usize) -> usize {
        for token_index in (0..filebuffer.tokens.len()).rev() {
            if filebuffer.tokens[token_index].index == index {
                return token_index;
            }

            if filebuffer.tokens[token_index].index < index {
                return token_index;
            }
        }

        panic!("index {} was out of bounds", index);
    }

    fn is_single_word_selected(&self) -> bool {
        return self.selections.len() == 1 || self.adding_selection && !self.mode.is_line();
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

    fn token_mode(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        if !self.mode.is_token() {
            self.set_selection_mode(filebuffer, SelectionMode::Token);

            // CAPTURE ENTIRE SELECTION AS WORDS
            //for index in 0..self.selections.len() {
            //    self.selections[index].reset();
            //    let token_index = self.token_from_index(self.selections[index].index);
            //    let new_index = filebuffer.tokens[token_index].index;
            //    let length = filebuffer.tokens[token_index].length;
            //    self.selections[index].set_index_length(new_index, length);
            //}

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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
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

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context, filebuffer);
    }

    fn do_buffer_action(&mut self, action: BufferAction) {
        match action {
            BufferAction::AddSelection(window_id, index, primary_index, secondary_index, offset) => {
                let selection = Selection::new(primary_index, secondary_index, offset);
                match index == self.selections.len() {
                    true => self.selections.push(selection),
                    false => self.selections.insert(index, selection),
                }
            },
            BufferAction::RemoveSelection(window_id, index, ..) => { self.selections.remove(index); },
            BufferAction::ChangePrimaryIndex(window_id, index, _previous, new) => self.selections[index].primary_index = new,
            BufferAction::ChangeSecondaryIndex(window_id, index, _previous, new) => self.selections[index].secondary_index = new,
            BufferAction::ChangeOffset(window_id, index, _previous, new) => self.selections[index].offset = new,
            BufferAction::ChangeSelectionMode(window_id, _previous, new) => self.mode = new,
            invalid => panic!("buffer action {:?} may not be executed", invalid),
        }
    }

    pub fn history_catch_up(&mut self, filebuffer: &mut Filebuffer) -> bool {
        let history_index = filebuffer.get_history_index();
        let force_rerender = self.history_index != history_index;

        while self.history_index > history_index {
            self.history_index -= 1;
            let action = filebuffer.get_action(self.history_index);

            if action.is_selection(self.window_id) {
                self.do_buffer_action(action.invert());
            }
        }

        while self.history_index < history_index {
            let action = filebuffer.get_action(self.history_index);
            self.history_index += 1;

            if action.is_selection(self.window_id) {
                self.do_buffer_action(action);
            }
        }

        return force_rerender;
    }

    fn undo(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        filebuffer.undo(language_manager);
        if self.history_catch_up(filebuffer) {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }
    }

    fn redo(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer) {
        filebuffer.redo(language_manager);
        if self.history_catch_up(filebuffer) {
            self.check_selection_gaps(textbuffer_context, filebuffer);
        }
    }

    pub fn select_last_character(&mut self, filebuffer: &mut Filebuffer) {
        for _index in 0..self.selections.len() - 1 {
            self.remove_selection(filebuffer, 1);
        }

        self.adding_selection = false;
        self.move_selection_to_end(filebuffer, 0);
        self.update_offset(filebuffer, 0);
        self.reset_selection(filebuffer, 0);
    }

    pub fn handle_action(&mut self, textbuffer_context: &TextbufferContext, language_manager: &mut LanguageManager, filebuffer: &mut Filebuffer, action: Action) -> Option<Action> {
        match action {

            Action::CharacterMode => handle_return!(self.character_mode(filebuffer)),

            Action::TokenMode => handle_return!(self.token_mode(textbuffer_context, filebuffer)),

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

        /*let mut offset = 0;
        while offset < current_length && offset < new_length {
            self.text[current_index] = new_text[offset];
            current_index += 1;
            offset += 1;
        }

        if current_length > new_length {
            self.remove_text_with_length(filebuffer, current_index, current_length - offset);
            self.unadvance_selections(index, current_length - new_length);
        } else if current_length < new_length {
            for offset in offset..new_length {
                self.text.insert(current_index, new_text[offset]);
                current_index += 1;
            }
            self.advance_selections(index, new_length - current_length);
        }*/
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

        self.adding_selection = false;
        self.character_mode(filebuffer);
        filebuffer.retokenize(language_manager);
    }


    fn render_text(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer) {

        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        let line_padding = (line_scaling - interface_context.font_size as f32) / 2.0 - (interface_context.font_size as f32 / 7.0);
        let line_number_width = theme.line_number_width as f32 * character_scaling;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.line_number_width as f32 * character_scaling + theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };

        let mut token_index = 0;
        let mut render_newline = true;
        let mut line_number = self.vertical_scroll;
        let mut index = self.index_from_line(filebuffer, self.vertical_scroll);
        let mut top_offset = theme.offset.y * interface_context.font_size as f32;
        let mut left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;

        let line_number_height = line_scaling - theme.line_number_gap * 2.0 * line_scaling;
        let line_number_size = Vector2f::new(theme.line_number_width * character_scaling, line_number_height);
        let character_size = Vector2f::new(character_scaling, line_scaling);

        let mut text_theme = match textbuffer_context.highlighting {
            true => filebuffer.tokens[token_index].get_theme(&theme),
            false => &theme.text_theme,
        };

        if textbuffer_context.line_numbers {
            let position = self.position + Vector2f::new(theme.line_number_offset * interface_context.font_size as f32, top_offset + theme.line_number_gap * line_scaling);
            Textfield::render(framebuffer, interface_context, &theme.line_number_theme, &format_shared!("{}", line_number), line_number_size, position, line_number_size.y);
        }

        for index in index..filebuffer.length() {

            if top_offset >= self.size.y {
                break;
            }

            while index >= filebuffer.tokens[token_index].index + filebuffer.tokens[token_index].length {
                token_index += 1;
                if textbuffer_context.highlighting {
                    text_theme = filebuffer.tokens[token_index].get_theme(&theme);
                }
            }

            if filebuffer.character(index).is_newline() && index != filebuffer.last_buffer_index() {
                left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
                top_offset += line_scaling;
                line_number += 1;

                if textbuffer_context.line_numbers {
                    let position = self.position + Vector2f::new(theme.line_number_offset * interface_context.font_size as f32, top_offset + theme.line_number_gap * line_scaling);
                    Textfield::render(framebuffer, interface_context, &theme.line_number_theme, &format_shared!("{}", line_number), line_number_size, position, line_number_size.y);
                }

                continue;
            }

            if left_offset <= self.size.x {
                let character_position = self.position + Vector2f::new(left_offset, top_offset);
                Text::render(framebuffer, interface_context, text_theme, &filebuffer.character(index).to_string(), character_size, character_position);
                left_offset += character_scaling;
            }
        }
    }

    fn render_selection_lines(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer) {

        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
        let line_scaling = interface_context.line_spacing * interface_context.font_size as f32;
        let scroll_offset = self.vertical_scroll as f32 * line_scaling + theme.offset.y * interface_context.font_size as f32;
        let line_number_offset = match textbuffer_context.line_numbers {
            true => theme.line_number_width as f32 * character_scaling + theme.line_number_offset * interface_context.font_size as f32,
            false => 0.0,
        };
        let mut left_offset = line_number_offset + theme.offset.x * interface_context.font_size as f32;
        let line_size = Vector2f::new(self.size.x, line_scaling);

        for index in 0..self.selections.len() {
            let start_index = self.selection_smallest_index(index);
            let mut top_offset = self.line_number_from_index(filebuffer, start_index) as f32 * line_scaling + theme.offset.y * interface_context.font_size as f32;
            let mut render_line = true;

            for offset in 0..self.selection_length(index) {
                if top_offset >= scroll_offset {
                    if render_line {
                        let position = self.position + Vector2f::new(left_offset, top_offset - scroll_offset + theme.offset.y * interface_context.font_size as f32);
                        Field::render(framebuffer, interface_context, &theme.selection_line_theme, line_size, position, line_scaling);
                        render_line = false;
                    }
                } else if top_offset - scroll_offset > self.size.y {
                    break;
                }

                if start_index + offset < filebuffer.length() && filebuffer.character(start_index + offset).is_newline() {
                    top_offset += line_scaling;
                    render_line = true;
                }
            }
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

    fn render_status_bar(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &TextfieldTheme, filebuffer: &Filebuffer) {

        //let status_bar_height = theme.height * interface_context.font_size as f32;
        //let size = Vector2f::new(self.size.x, status_bar_height);
        //let position = Vector2f::new(0.0, self.size.y - status_bar_height);
        //Textfield::render(framebuffer, interface_context, theme, SharedString::from(""), size, position);

        //let mut offset = self.area.offset + self.size.width;
        //terminal.set_color_pair(&context.theme.panel.background, &context.theme.overlay_color, true);

        //offset -= self.mode.name().len() + 3;
        //terminal.move_cursor(self.size.y - 1, offset);
        //print!(" {} ", self.mode.name());

        //offset -= self.language.len() + 3;
        //terminal.move_cursor(self.size.y - 1, offset);
        //print!(" {} ", self.language);

        //if self.is_single_word_selected() {
        //    let token_index = self.token_from_index(self.selections[self.selections.len() - 1].index);
        //    if let Some(display_name) = filebuffer.tokens[token_index].display_name() {
        //        offset -= display_name.len() + 3;
        //        terminal.move_cursor(self.size.y - 1, offset);
        //        print!(" {} ", display_name);
        //    }

        //    if let TokenType::Invalid(error) = &filebuffer.tokens[token_index].token_type {
        //        let error_string = format!("{:?}", error); // TEMP
        //        offset -= error_string.len() + 3;
        //        terminal.set_color_pair(&context.theme.panel.background, &context.theme.error_color, true);
        //        terminal.move_cursor(self.size.y - 1, offset);
        //        print!(" {} ", error_string);
        //    }
        //}
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &TextbufferTheme, filebuffer: &Filebuffer, scaler: f32, focused: bool) {
        Field::render(framebuffer, interface_context, &theme.background_theme, self.size, self.position, scaler);

        if textbuffer_context.selection_lines && (focused || textbuffer_context.unfocused_selections) {
            self.render_selection_lines(framebuffer, interface_context, textbuffer_context, theme, filebuffer);
        }

        self.render_text(framebuffer, interface_context, textbuffer_context, theme, filebuffer);

        if focused || textbuffer_context.unfocused_selections {
            self.render_selections(framebuffer, interface_context, textbuffer_context, theme, filebuffer);
        }

        if textbuffer_context.status_bar && focused {
            self.render_status_bar(framebuffer, interface_context, &theme.status_bar_theme, filebuffer);
        }
    }
}
