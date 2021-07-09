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
use system::Filebuffer;

use self::selection::SelectionMode;
use self::token::Token;

pub use self::selection::{ Selection, SelectionTheme };
pub use self::theme::TextbufferTheme;
pub use self::context::TextbufferContext;

macro_rules! handle_return {
    ($expression: expr) => ({
        $expression;
        return None;
    })
}

pub fn length_from_position(position: Vec<Position>) -> usize {
    return position.iter().map(|position| position.length).sum();
}

pub struct Textbuffer {
    //text: SharedString,
    tokenizer: Tokenizer,
    language: SharedString,
    tokens: Vec<Token>,
    selections: Vec<Selection>,
    mode: SelectionMode,
    adding_selection: bool,
    padding: Character,
    size: Vector2f,
    position: Vector2f,
    vertical_scroll: usize,
    horizontal_scroll: usize,
    selection_lines: bool,
    status_bar: bool,
    multiline: bool,
    history_index: usize,
}

impl Textbuffer {

    pub fn new(size: Vector2f, position: Vector2f, padding: char, selection_lines: bool, status_bar: bool, multiline: bool) -> Self {
        let language = SharedString::from("none");

        Self {
            //text: format_shared!("{}", padding),
            tokenizer: guaranteed!(Self::load_language(&language)),
            language: language,
            tokens: vec![Token::new(TokenType::Operator(SharedString::from("newline")), 0, 1)],
            selections: vec![Selection::new(0, 0, 0)],
            mode: SelectionMode::Character,
            adding_selection: false,
            padding: Character::from_char(padding),
            size: size,
            position: position,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            selection_lines: selection_lines,
            status_bar: status_bar,
            multiline: multiline,
            history_index: 0,
        }
    }

    pub fn set_text(&mut self, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text(text);
        self.reset();
        return self.parse(filebuffer);
    }

    pub fn set_text_without_save(&mut self, filebuffer: &mut Filebuffer, text: SharedString) -> Status<()> {
        filebuffer.set_text_without_save(text);
        self.reset();
        return self.parse(filebuffer);
    }

    pub fn resize(&mut self, size: Vector2f) {
        self.size = size;
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.position = position;
    }

    pub fn reset(&mut self) {
        self.vertical_scroll = 0;
        self.horizontal_scroll = 0;
        self.selections = vec![Selection::new(0, 0, 0)];
        self.adding_selection = false;
        self.mode = SelectionMode::Character;
    }

    fn load_language(language: &SharedString) -> Status<Tokenizer> {
        let file_path = format_shared!("/home/.config/poet/languages/{}.data", language);
        let tokenizer_map = confirm!(read_map(&file_path)); // confirm!(read_map(&file_path), Message, "...");
        return Tokenizer::new(&tokenizer_map);
    }

    pub fn parse(&mut self, filebuffer: &Filebuffer) -> Status<()> {

        let (mut token_stream, registry, notes) = display!(self.tokenizer.tokenize(filebuffer.get_text(), None, true));
        let mut tokens = Vec::new();
        let mut offset = 0;

        for token in token_stream.into_iter() {
            let length = length_from_position(token.position);
            tokens.push(Token::new(token.token_type, offset, length));
            offset += length;
        }

        self.tokens = tokens;
        return success!(());
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

    fn visible_line_count(&self) -> usize {
        //let line_scaling = context.line_spacing * context.font_size as f32;
        //return (self.size.y / line_scaling) as usize;

        return 40;
        //return self.visible_line_count;
    }

    fn check_selection_gaps(&mut self, textbuffer_context: &TextbufferContext) {

        if !self.multiline {
            return;
        }

        /*let selection = self.selections.last().unwrap();
        let line_number = self.line_number_from_index(selection.primary_index);
        let visible_line_count = self.visible_line_count() - 1;

        if line_number < self.vertical_scroll + context.selection_gap {
            match line_number > context.selection_gap {
                true => self.vertical_scroll = line_number - context.selection_gap,
                false => self.vertical_scroll = 0,
            }
        } else if line_number + context.selection_gap > self.vertical_scroll + visible_line_count {
            self.vertical_scroll += line_number + context.selection_gap - self.vertical_scroll - visible_line_count;
        }*/
    }

    fn move_selection_left(&mut self, index: usize) -> bool {
        if self.selections[index].primary_index > 0 {
            self.selections[index].primary_index -= 1;
            return true;
        }
        return false;
    }

    fn move_selection_right(&mut self, filebuffer: &Filebuffer, index: usize) -> bool {
        if self.selections[index].primary_index < filebuffer.last_buffer_index() {
            self.selections[index].primary_index += 1;
            return true;
        }
        return false;
    }

    fn move_selection_down(&mut self, filebuffer: &Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;

        for current_index in primary_index..filebuffer.last_buffer_index() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.line_length_from_index(filebuffer, current_index + 1);
                let distance_to_offset = min(line_length, self.selections[index].offset + 1);
                self.selections[index].primary_index = current_index + distance_to_offset;
                return;
            }
        }

        self.selections[index].primary_index = filebuffer.last_buffer_index();
    }

    fn move_selection_up(&mut self, filebuffer: &Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;

        for current_index in (0..primary_index).rev() {
            if filebuffer.character(current_index).is_newline() {
                let line_length = self.reverse_line_length_from_index(filebuffer, current_index) - 1;
                let distance_to_offset = line_length - min(line_length, self.selections[index].offset);
                self.selections[index].primary_index = current_index - distance_to_offset;
                return;
            }
        }

        self.selections[index].primary_index = 0;
    }

    fn move_selection_to_end(&mut self, filebuffer: &Filebuffer, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let distance_to_newline = self.line_length_from_index(filebuffer, primary_index);
        self.selections[index].primary_index += distance_to_newline - 1;
    }

    fn move_selection_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer, complete: bool, index: usize) {
        let primary_index = self.selections[index].primary_index;
        let distance_to_newline = self.reverse_line_length_from_index(filebuffer, primary_index);
        let adjusted_index = self.adjust_start_index(textbuffer_context, filebuffer, complete, primary_index, distance_to_newline - 1);
        self.selections[index].primary_index = adjusted_index;
    }

    fn move_secondary_to_end(&mut self, filebuffer: &Filebuffer, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let distance_to_newline = self.line_length_from_index(filebuffer, secondary_index);
        self.selections[index].secondary_index += distance_to_newline - 1;
    }

    fn move_secondary_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer, complete: bool, index: usize) {
        let secondary_index = self.selections[index].secondary_index;
        let distance_to_newline = self.reverse_line_length_from_index(filebuffer, secondary_index);
        let adjusted_index = self.adjust_start_index(textbuffer_context, filebuffer, complete, secondary_index, distance_to_newline - 1);
        self.selections[index].secondary_index = adjusted_index;
    }

    fn selection_smallest_index(&self, index: usize) -> usize {
        return min(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn selection_biggest_index(&self, index: usize) -> usize {
        return max(self.selections[index].primary_index, self.selections[index].secondary_index);
    }

    fn move_selection_to_first(&mut self, index: usize) {
        self.selections[index].primary_index = self.selection_smallest_index(index);
    }

    fn move_selection_to_last(&mut self, index: usize) {
        self.selections[index].primary_index = self.selection_biggest_index(index);
    }

    fn is_selection_extended(&self, index: usize) -> bool {
        return self.selections[index].primary_index != self.selections[index].secondary_index;
    }

    fn is_selection_inverted(&self, index: usize) -> bool {
        return self.selections[index].primary_index < self.selections[index].secondary_index;
    }

    fn update_offset(&mut self, filebuffer: &Filebuffer, index: usize) {
        self.selections[index].offset = self.offset_from_index(filebuffer, self.selections[index].primary_index);
    }

    fn reset_selection(&mut self, index: usize) {
        self.selections[index].secondary_index = self.selections[index].primary_index;
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

    fn selection_exclude_last(&mut self, index: usize) {
        match self.is_selection_inverted(index) {
            true => self.selections[index].secondary_index -= 1,
            false => self.selections[index].primary_index -= 1,
        }
    }

    fn validate_text(&mut self, filebuffer: &mut Filebuffer) {
        if filebuffer.is_empty() || filebuffer.last_character() != self.padding {
            self.insert_text(filebuffer, filebuffer.last_buffer_index(), self.padding.to_string());
        }
    }

    fn insert_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, text: SharedString) {
        self.history_index = filebuffer.insert_text(buffer_index, text);
    }

    fn remove_text(&mut self, filebuffer: &mut Filebuffer, buffer_index: usize, length: usize) {
        self.history_index = filebuffer.remove_text(buffer_index, length);
        self.validate_text(filebuffer);
    }

    fn clip_selection(&mut self, filebuffer: &Filebuffer, index: usize) {
        self.selections[index].primary_index = min(self.selections[index].primary_index, filebuffer.last_buffer_index());
    }

    fn set_selection_length(&mut self, index: usize, length: usize) {
        let last_index = self.selection_smallest_index(index) + length - 1;

        match self.is_selection_inverted(index) {
            true => self.selections[index].secondary_index = last_index,
            false => self.selections[index].primary_index = last_index,
        }
    }

    fn delete_selected(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let buffer_index = self.selection_smallest_index(index);
        let selection_length = self.selection_length(index);
        self.remove_text(filebuffer, buffer_index, selection_length);
        self.move_selection_to_first(index);
        self.clip_selection(filebuffer, index);
        self.update_offset(filebuffer, index);
        self.reset_selection(index);
        self.unadvance_selections(filebuffer, index, selection_length);
    }

    fn delete_selected_primary(&mut self, filebuffer: &mut Filebuffer, index: usize) {
        let buffer_index = self.selections[index].primary_index;
        self.remove_text(filebuffer, buffer_index, 1);
        self.clip_selection(filebuffer, index);
        self.update_offset(filebuffer, index);
        self.reset_selection(index);
        self.unadvance_selections(filebuffer, index, 1);
    }

    fn is_selection_end_of_buffer(&self, filebuffer: &Filebuffer, index: usize) -> bool {
        return self.selections[index].primary_index == filebuffer.last_buffer_index();
    }

    fn append(&mut self, filebuffer: &Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_last(index);

            if !self.is_selection_newline(filebuffer, index) {
                self.move_selection_right(filebuffer, index);
            }

            self.update_offset(filebuffer, index);
            self.reset_selection(index);
        }

        self.mode = SelectionMode::Character;
        //self.merge_overlapping_selections();
    }

    fn insert(&mut self, filebuffer: &Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_first(index);
            self.update_offset(filebuffer, index);
            self.reset_selection(index);
        }

        self.mode = SelectionMode::Character;
        //self.merge_overlapping_selections();
    }

    fn newline_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_first(index);
            self.move_selection_to_start(textbuffer_context, filebuffer, true, index);
            self.update_offset(filebuffer, index);
            self.reset_selection(index);
        }

        self.mode = SelectionMode::Character;
        //self.merge_overlapping_selections();

        for index in self.selection_start()..self.selections.len() {
            self.insert_text(filebuffer, self.selections[index].primary_index, SharedString::from("\n"));
            self.advance_selections(filebuffer, index, 1);
        }

        self.parse(filebuffer);
    }

    fn newline_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        for index in self.selection_start()..self.selections.len() {
            self.move_selection_to_last(index);
            self.move_selection_to_end(filebuffer, index);
        }

        self.mode = SelectionMode::Character;
        //self.merge_overlapping_selections();

        for index in self.selection_start()..self.selections.len() {
            let newline_index = self.selections[index].primary_index + 1;
            self.insert_text(filebuffer, newline_index, SharedString::from("\n"));
            self.advance_selections(filebuffer, index, 1);
            self.move_selection_right(filebuffer, index);
            self.update_offset(filebuffer, index);
            self.reset_selection(index);
        }

        self.check_selection_gaps(textbuffer_context);
        self.parse(filebuffer);
    }

    fn remove(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    if self.is_selection_extended(index) {
                        self.delete_selected(filebuffer, index);
                    } else if self.move_selection_left(index) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
        self.parse(filebuffer);
    }

    fn delete(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
        self.parse(filebuffer);
    }

    fn delete_line(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
        self.parse(filebuffer);
    }

    fn move_left(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_extended(index) {
                        true => self.move_selection_to_first(index),
                        false => { self.move_selection_left(index); },
                    }
                    self.update_offset(filebuffer, index);
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => return,
        }

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn move_right(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_extended(index) {
                        true => self.move_selection_to_last(index),
                        false => { self.move_selection_right(filebuffer, index); },
                    }
                    self.update_offset(filebuffer, index);
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => return,
        }

        //self.check_selection_gaps(context);
        //self.merge_overlapping_selections();
    }

    fn move_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_first(index),
                        false => self.move_selection_up(filebuffer, index),
                    }
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_first(index),
                        false => self.move_selection_up(filebuffer, index),
                    }
                    self.reset_selection(index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn move_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_last(index),
                        false => self.move_selection_down(filebuffer, index),
                    }
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    match self.is_selection_multiline(filebuffer, index) {
                        true => self.move_selection_to_last(index),
                        false => self.move_selection_down(filebuffer, index),
                    }
                    self.reset_selection(index);
                    self.move_secondary_to_start(textbuffer_context, filebuffer, true, index);
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                }
            },
        }

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn extend_left(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_left(index);
                    self.update_offset(filebuffer, index);
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => return,
        }

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn extend_right(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn extend_up(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn extend_down(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
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

        self.check_selection_gaps(textbuffer_context);
        //self.merge_overlapping_selections();
    }

    fn move_to_end(&mut self, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_to_end(filebuffer, index);
                    self.update_offset(filebuffer, index);
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn move_to_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.move_selection_to_start(textbuffer_context, filebuffer, false, index);
                    self.update_offset(filebuffer, index);
                    self.reset_selection(index);
                }
            },

            SelectionMode::Token => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn extend_end(&mut self, filebuffer: &Filebuffer) {
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

    fn extend_start(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
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

    fn token_from_index(&self, index: usize) -> usize {
        for token_index in (0..self.tokens.len()).rev() {
            if self.tokens[token_index].index == index {
                return token_index;
            }

            if self.tokens[token_index].index < index {
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

    fn character_mode(&mut self) {
        if !self.mode.is_character() {
            self.mode = SelectionMode::Character;
        }
    }

    fn token_mode(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        if !self.mode.is_token() {
            self.mode = SelectionMode::Token;

            // CAPTURE ENTIRE SELECTION AS WORDS
            //for index in 0..self.selections.len() {
            //    self.selections[index].reset();
            //    let token_index = self.token_from_index(self.selections[index].index);
            //    let new_index = self.tokens[token_index].index;
            //    let length = self.tokens[token_index].length;
            //    self.selections[index].set_index_length(new_index, length);
            //}

            //self.merge_overlapping_selections();
        }
    }

    fn line_mode(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        if !self.mode.is_line() {
            self.mode = SelectionMode::Line;

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

    fn add_selection(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
        match self.mode {

            SelectionMode::Character => {
                let buffer_index = self.selection_biggest_index(self.selections.len() - 1) + 1;
                let offset = self.offset_from_index(filebuffer, buffer_index);
                let new_selection = Selection::new(buffer_index, buffer_index, offset);
                self.selections.push(new_selection);
                self.adding_selection = true;
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
                let buffer_index = self.selection_biggest_index(self.selections.len() - 1) + 1;
                let offset = self.offset_from_index(filebuffer, buffer_index);
                let new_selection = Selection::new(buffer_index, buffer_index, offset);
                self.selections.push(new_selection);
                self.adding_selection = true;

                self.reset_selection(self.selections.len() - 1);
                self.move_selection_to_end(filebuffer, self.selections.len() - 1);
            },
        }

        self.check_selection_gaps(textbuffer_context);
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

    fn select_next(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &Filebuffer) {
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
                        self.selections.push(selection);
                    }
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context);
    }

    fn duplicate_up(&mut self, textbuffer_context: &TextbufferContext) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {

                    /*let selection_length = self.selection_length(index);
                    let selection_buffer = self.get_selected_text(index);
                    let mut selection_matches = self.text.position(&selection_buffer);

                    self.sort_selection_matches(index, &mut selection_matches);
                    let primary_index = selection_matches[0];
                    let secondary_index = primary_index + selection_length - 1;

                    if !self.index_has_selection(primary_index, secondary_index) {
                        let offset = self.offset_from_index(primary_index);
                        let selection = Selection::new(primary_index, secondary_index, offset);
                        self.selections.push(selection);
                    }*/
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context);
    }

    fn duplicate_down(&mut self, textbuffer_context: &TextbufferContext) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
            },
        }

        self.check_selection_gaps(textbuffer_context);
    }

    pub fn select_last_character(&mut self, filebuffer: &Filebuffer) {
        for _index in 0..self.selections.len() - 1 {
            self.selections.remove(1);
        }

        self.adding_selection = false;
        self.move_selection_to_end(filebuffer, 0);
        self.update_offset(filebuffer, 0);
        self.reset_selection(0);
    }

    pub fn handle_action(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer, action: Action) -> Option<Action> {
        match action {

            Action::CharacterMode => handle_return!(self.character_mode()),

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

            Action::DuplicateUp => handle_return!(self.duplicate_up(textbuffer_context)),

            Action::DuplicateDown => handle_return!(self.duplicate_down(textbuffer_context)),

            Action::Append => handle_return!(self.append(filebuffer)),

            Action::Insert => handle_return!(self.insert(filebuffer)),

            Action::NewlineUp => handle_return!(self.newline_up(textbuffer_context, filebuffer)),

            Action::NewlineDown => handle_return!(self.newline_down(textbuffer_context, filebuffer)),

            Action::AddSelection => handle_return!(self.add_selection(textbuffer_context, filebuffer)),

            Action::SelectNext => handle_return!(self.select_next(textbuffer_context, filebuffer)),

            Action::Remove => handle_return!(self.remove(textbuffer_context, filebuffer)),

            Action::Delete => handle_return!(self.delete(textbuffer_context, filebuffer)),

            Action::DeleteLine => handle_return!(self.delete_line(textbuffer_context, filebuffer)),

            Action::Rotate => handle_return!(self.rotate_selections(filebuffer)),

            Action::Undo => handle_return!(self.history_index = filebuffer.undo()),

            Action::Redo => handle_return!(self.history_index = filebuffer.redo()),

            Action::Abort => {
                if self.selections.len() > 1 {
                    self.drop_selections(textbuffer_context);
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

        println!("{} --- {}", current_text.serialize(), new_text.serialize());

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

    fn rotate_selections(&mut self, filebuffer: &mut Filebuffer) {
        if self.selections.len() > 1 {
            let mut buffer = self.get_selected_text(filebuffer, self.selections.len() - 1);

            for index in 0..self.selections.len() {
                let new_text = self.get_selected_text(filebuffer, index);
                let new_length = buffer.len();

                self.replace_selected_text(filebuffer, index, buffer);
                self.set_selection_length(index, new_length);
                self.update_offset(filebuffer, index);
                buffer = new_text;
            }

            self.validate_text(filebuffer);
            self.parse(filebuffer);
        }
    }

    fn clear_selections(&mut self) {
        self.selections.clear();
        self.adding_selection = false;
    }

    fn drop_selections(&mut self, textbuffer_context: &TextbufferContext) {
        for _index in 0..self.selections.len() - 1 {
            self.selections.remove(1);
        }

        // reset that selection
        self.adding_selection = false;
        self.check_selection_gaps(textbuffer_context);
    }

    fn set_selections_from_string(&mut self, filebuffer: &Filebuffer, string: &SharedString) {

        self.clear_selections();

        let positions = filebuffer.get_text().position(string);
        let length = string.len();

        for index in positions {

            let offset = self.offset_from_index(filebuffer, index);
            let selection = Selection::new(index, length, offset);

            self.selections.push(selection);
        }
    }

    pub fn add_character(&mut self, textbuffer_context: &TextbufferContext, filebuffer: &mut Filebuffer, character: Character) {

        for index in 0..self.selections.len() {
            if self.is_selection_extended(index) {

                if textbuffer_context.preserve_lines && self.is_last_selected_newline(filebuffer, index) {
                    self.selection_exclude_last(index);
                }

                self.replace_selected_text(filebuffer, index, character.to_string());
                self.move_selection_to_first(index);
                self.move_selection_right(filebuffer, index);
                self.update_offset(filebuffer, index);
                self.reset_selection(index);
            } else {
                let buffer_index = self.selections[index].primary_index;
                self.insert_text(filebuffer, buffer_index, character.to_string());
                self.move_selection_right(filebuffer, index);
                self.update_offset(filebuffer, index);
                self.reset_selection(index);
                self.advance_selections(filebuffer, index, 1);
            }
        }

        self.adding_selection = false;
        self.character_mode();
        self.parse(filebuffer);
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

        //if context.highlighting {
        //    character_text.set_fill_color(self.tokens[token_index].get_color(context));
        //    character_text.set_style(self.tokens[token_index].get_style(context));
        //}

        if textbuffer_context.line_numbers {
            let position = self.position + Vector2f::new(theme.line_number_offset * interface_context.font_size as f32, top_offset + theme.line_number_gap * line_scaling);
            Textfield::render(framebuffer, interface_context, &theme.line_number_theme, &format_shared!("{}", line_number), line_number_size, position, line_number_size.y);
        }

        for index in index..filebuffer.length() {

            if top_offset >= self.size.y {
                break;
            }

            //while index >= self.tokens[token_index].index + self.tokens[token_index].length {
            //    token_index += 1;
            //    if context.highlighting {
            //        character_text.set_fill_color(self.tokens[token_index].get_color(context));
            //        character_text.set_style(self.tokens[token_index].get_style(context));
            //    }
            //}

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
                Text::render(framebuffer, interface_context, &theme.text_theme, &filebuffer.character(index).to_string(), character_size, character_position);
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
        //    if let Some(display_name) = self.tokens[token_index].display_name() {
        //        offset -= display_name.len() + 3;
        //        terminal.move_cursor(self.size.y - 1, offset);
        //        print!(" {} ", display_name);
        //    }

        //    if let TokenType::Invalid(error) = &self.tokens[token_index].token_type {
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
