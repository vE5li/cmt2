mod selection;
mod mode;
mod textbox;
mod combobox;
mod filebox;

use kami::*;
use kami::tokenize::tokenize;
//use parse::parse;

use sfml::graphics::*;
use sfml::system::Vector2f;
use self::selection::Selection;
use self::mode::SelectionMode;
use self::textbox::TextBox;
use self::combobox::{ ComboBox, ComboSelection };
use self::filebox::FileBox;
use context::{ Context, Action };
use graphics::{ RoundedRectangle, draw_spaced_text };










#[derive(Clone, Debug)]
pub struct EditorToken {
    pub token_type: TokenType,
    pub index: usize,
    pub length: usize,
}

impl EditorToken {

    pub fn new(token_type: TokenType, index: usize, length: usize) -> Self {
        Self {
            token_type: token_type,
            length: length,
            index: index,
        }
    }

    pub fn display_name(&self) -> Option<&'static str> {
        match self.token_type {
            TokenType::Comment(..) => return Some("comment"),
            TokenType::Operator(..) => return Some("operator"),
            TokenType::Keyword(..) => return Some("keyword"),
            TokenType::Identifier(..) => return Some("identifier"),
            TokenType::TypeIdentifier(..) => return Some("type identifier"),
            TokenType::Character(..) => return Some("character"),
            TokenType::String(..) => return Some("string"),
            TokenType::Integer(..) => return Some("integer"),
            TokenType::Float(..) => return Some("float"),
            TokenType::Invalid(..) => return None,
            TokenType::Ignored => return None,
        }
    }

    pub fn get_color(&self, context: &Context) -> Color {
        match self.token_type {
            TokenType::Comment(..) => return context.theme.panel.comment,
            TokenType::Operator(..) => return context.theme.panel.operator,
            TokenType::Keyword(..) => return context.theme.panel.keyword,
            TokenType::Identifier(..) => return context.theme.panel.identifier,
            TokenType::TypeIdentifier(..) => return context.theme.panel.type_identifier,
            TokenType::Character(..) => return context.theme.panel.character,
            TokenType::String(..) => return context.theme.panel.string,
            TokenType::Integer(..) => return context.theme.panel.integer,
            TokenType::Float(..) => return context.theme.panel.float,
            TokenType::Invalid(..) => return context.theme.panel.error,
            TokenType::Ignored => return context.theme.panel.text,
        }
    }
}

pub fn length_from_position(position: Vec<Position>) -> usize {
    return position.iter().map(|position| position.length).sum();
}



pub struct OpenFileDialogue {
    pub file_name_box: FileBox,
}

impl OpenFileDialogue {

    pub fn new() -> Self {
        Self {
            file_name_box: FileBox::new("open file", " > ", 0, false),
        }
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {

        if let Action::OpenFile = action {
            return (true, Some(false));
        }

        return self.file_name_box.handle_action(context, action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.file_name_box.add_character(character);
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, width: f32) {
        self.file_name_box.draw(framebuffer, context, width, 0, true);
    }
}

pub struct SetLanguageDialogue {
    pub language_box: ComboBox,
}

impl SetLanguageDialogue {

    pub fn new(language: VectorString) -> Self {
        Self {
            language_box: ComboBox::new("select language", " > ", 0, false, false, vec![VectorString::from("cipher"), VectorString::from("c++"), VectorString::from("default"), VectorString::from("doofenshmirtz"), VectorString::from("entleman"), VectorString::from("none"), VectorString::from("rust"), VectorString::from("seamonkey")]),
        }
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {

        if let Action::SetLanguage = action {
            return (true, Some(false));
        }

        return self.language_box.handle_action(context, action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.language_box.add_character(character);
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, width: f32) {
        self.language_box.draw(framebuffer, context, width, 0, true);
    }
}

pub fn fill_line(length: usize, character: char) {
    for _index in 0..length {
        print!("{}", character);
    }
}

#[derive(Clone)]
pub enum DialogueMode {
    None,
    Error(VectorString),
    OpenFile,
    SetLanguage,
}

impl DialogueMode {

    pub fn selections_shown(&self) -> bool {
        match self {
            DialogueMode::None => return true,
            DialogueMode::Error(..) => return true,
            _other => return false,
        }
    }
}






macro_rules! handle_return {
    ($expression: expr) => ({
        $expression;
        return success!(true);
    })
}

pub struct Editor {
    file_name: Option<VectorString>,
    content: VectorString,
    compiler: Data,
    language: VectorString,
    tokens: Vec<EditorToken>,
    selections: Vec<Selection>,
    mode: SelectionMode,
    adding_selection: bool,
    size: Vector2f,
    scroll: usize,
    font_size: usize,

    open_file_dialogue: OpenFileDialogue,
    set_language_dialogue: SetLanguageDialogue,
    dialogue_mode: DialogueMode,
}

// TOKEN TYPE IN THE STATUS BAR

impl Editor {

    pub fn new(font_size: usize) -> Status<Self> {
        let language = VectorString::from("none");
        success!(Self {
            file_name: None,
            content: VectorString::from("\n"),
            compiler: confirm!(Self::load_language(&language)),
            language: language.clone(),
            tokens: vec![EditorToken::new(TokenType::Operator(VectorString::from("newline")), 0, 1)],
            selections: vec![Selection::new(0, 1, 0)],
            mode: SelectionMode::Character,
            adding_selection: false,
            size: Vector2f::new(0.0, 0.0),
            scroll: 0,
            font_size: font_size,

            open_file_dialogue: OpenFileDialogue::new(),
            set_language_dialogue: SetLanguageDialogue::new(language),
            dialogue_mode: DialogueMode::None,
        })
    }

    pub fn update_graphics(&mut self, context: &Context, size: Vector2f) {
        self.size = size;

        //match &mut self.mode {
        //    PanelMode::Editor(editor) => editor.update_graphics(context, size),
        //    PanelMode::Terminal => { },
        //}
    }

    pub fn open_file(&mut self, file_name: VectorString) -> Status<()> {

        // make sure there is no unsaved changes
        self.content = confirm!(read_file(&file_name));
        self.file_name = Some(file_name.clone()); // REMOVE CLONE
        self.reset();

        // make sure that the file ends on a newline and if not, append one

        // set language for specific file if specified and only load if it changed
        let pieces = file_name.split(&VectorString::from("."), true);
        self.language = if pieces.len() > 1 {
            match pieces.last().unwrap().printable().as_ref() {
                "rs" => VectorString::from("rust"),
                "cip" => VectorString::from("cipher"),
                "asm" => VectorString::from("doofenshmirtz"),
                "uni" => VectorString::from("entleman"),
                "data" => VectorString::from("seamonkey"),
                _other => VectorString::from("none"),
            }
        } else {
            VectorString::from("none")
        };

        //self.language = VectorString::from("none");
        self.compiler = confirm!(Self::load_language(&self.language));
        return self.parse();
    }

    pub fn reset(&mut self) {
        self.scroll = 0;
        self.selections = vec![Selection::new(0, 1, 0)];
        self.adding_selection = false;
        self.mode = SelectionMode::Character;
    }

    fn load_language(language: &VectorString) -> Status<Data> {
        let file_path = format_vector!("/home/.poet/languages/{}.data", language);
        return read_map(&file_path);
    }

    pub fn parse(&mut self) -> Status<()> {
        let (mut token_stream, registry) = display!(tokenize(&self.compiler, self.content.clone(), self.file_name.clone(), &None, &map!(), &map!(), true), &None, &map!(), &map!());
        let mut tokens = Vec::new();
        let mut offset = 0;

        for token in token_stream.into_iter() {
            let length = length_from_position(token.position);
            tokens.push(EditorToken::new(token.token_type, offset, length));
            offset += length;
        }

        self.tokens = tokens;
        return success!(());
    }

    fn move_left(&mut self, context: &Context) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    let previous_length = self.selections[index].length;
                    self.selections[index].reset();

                    if previous_length <= 1 && self.selections[index].index > 0 {
                        self.selections[index].index -= 1;
                        self.selections[index].offset = self.offset_from_index(self.selections[index].index);
                    }
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
        //self.check_selection_gap_up(context, self.selections.len() - 1);
    }

    fn move_right(&mut self, context: &Context) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    let previous_length = self.selections[index].length;
                    self.selections[index].reset();

                    if previous_length > 1 {
                        let new_index = self.selections[index].index + previous_length - 1;
                        self.selections[index].set_index_offset(new_index, new_index);
                    } else if self.selections[index].index < self.content.len() - 1 {
                        self.selections[index].index += 1;
                        self.selections[index].offset = self.offset_from_index(self.selections[index].index);
                    }
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
        //self.check_selection_gap_up(context, self.selections.len() - 1);
    }

    fn move_up(&mut self, context: &Context) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.selections[index].reset();

                    match self.line_from_index(self.selections[index].index) {
                        0 => self.selections[index].set_index_offset(0, 0),
                        line => self.selections[index].index = self.index_from_line_offset(line - 1, self.selections[index].offset),
                    }
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    let line = self.line_from_index(self.selections[index].index);
                    if line != 0 {
                        let new_index = self.index_from_line(line - 1);
                        let length = self.line_length_from_index(new_index);
                        self.selections[index].set_index_length(new_index, length);
                    }
                }
            },
        }

        //self.merge_overlapping_selections();
        //self.check_selection_gap_up(context, self.selections.len() - 1);
    }

    fn move_down(&mut self, context: &Context) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.selections[index].reset();

                    let line = self.line_from_index(self.selections[index].index);
                    if line == self.line_count() {
                        let new_index = self.content.len() - 1;
                        let offset = self.offset_from_index(new_index);
                        self.selections[index].set_index_offset(new_index, offset);
                    } else {
                        self.selections[index].index = self.index_from_line_offset(line + 1, self.selections[index].offset)
                    }
                }
            },

            SelectionMode::Token => {
            },

            SelectionMode::Line => {
                for index in self.selection_start()..self.selections.len() {
                    let line = self.line_from_index(self.selections[index].index);
                    if line != self.line_count() {
                        let new_index = self.index_from_line(line + 1);
                        let length = self.line_length_from_index(new_index);
                        self.selections[index].set_index_length(new_index, length);
                    }
                }
            },
        }

        //self.merge_overlapping_selections();
        //self.check_selection_gap_down(context, self.selections.len() - 1);
    }

    fn move_to_end(&mut self) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.selections[index].reset();
                    let remaining = self.line_length_from_index(self.selections[index].index);
                    let new_index = self.selections[index].index + remaining - 1;
                    let new_offset = self.offset_from_index(new_index);
                    self.selections[index].set_index_offset(new_index, new_offset);
                }
            },

            SelectionMode::Token => {

            },

            SelectionMode::Line => return,
        }

        //self.merge_overlapping_selections();
    }

    fn move_to_start(&mut self) {
        match self.mode {

            SelectionMode::Character => {
                for index in self.selection_start()..self.selections.len() {
                    self.selections[index].reset();
                    // move to the first character thats not a space?
                    let line = self.line_from_index(self.selections[index].index);
                    let new_index = self.index_from_line(line);
                    self.selections[index].set_index_offset(new_index, 0);
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

    fn line_count(&self) -> usize {
        return self.line_from_index(self.content.len() - 1);
    }

    fn index_from_line(&self, line: usize) -> usize {
        let mut line_count = 0;

        for index in 0..self.content.len() {
            if line_count == line {
                return index;
            }

            if self.content[index].is_newline() {
                line_count += 1;
            }
        }

        panic!("line index {} was out of bounds", line);
    }

    fn index_from_line_offset(&self, line: usize, offset: usize) -> usize {
        let mut line_count = 0;
        let mut character_count = 0;

        for index in 0..self.content.len() {
            if line_count == line && character_count == offset {
                return index;
            }

            if self.content[index].is_newline() {
                if line_count == line {
                    return index;
                }

                line_count += 1;
                character_count = 0;
            } else {
                character_count += 1;
            }
        }

        panic!("line index {} was out of bounds", line);
    }

    fn line_from_index(&self, index: usize) -> usize {
        let mut line_count = 0;

        for current_index in 0..self.content.len() {
            if current_index == index {
                return line_count;
            }

            if self.content[current_index].is_newline() {
                line_count += 1;
            }
        }

        panic!("index {} was out of bounds", index);
    }

    fn offset_from_index(&self, index: usize) -> usize {
        let mut left_offset = 0;

        for current_index in 0..self.content.len() {
            if current_index == index {
                return left_offset;
            }

            if self.content[current_index].is_newline() {
                left_offset = 0;
            } else {
                left_offset += 1;
            }
        }

        panic!("index {} was out of bounds", index);
    }

    fn line_length_from_index(&self, index: usize) -> usize {
        let mut length = 1;

        for current_index in index..self.content.len() {
            if self.content[current_index].is_newline() {
                return length;
            }
            length += 1;
        }

        panic!("index {} was out of bounds", index);
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

    fn character_mode(&mut self) {
        if !self.mode.is_character() {
            self.mode = SelectionMode::Character;
        }
    }

    fn token_mode(&mut self) {
        if !self.mode.is_token() {
            self.mode = SelectionMode::Token;

            // CAPTURE ENTIRE SELECTION AS WORDS
            for index in 0..self.selections.len() {
                self.selections[index].reset();
                let token_index = self.token_from_index(self.selections[index].index);
                let new_index = self.tokens[token_index].index;
                let length = self.tokens[token_index].length;
                self.selections[index].set_index_length(new_index, length);
            }

            //self.merge_overlapping_selections();
        }
    }

    fn line_mode(&mut self) {
        if !self.mode.is_line() {
            self.mode = SelectionMode::Line;

            for index in 0..self.selections.len() {
                self.selections[index].reset();
                let line = self.line_from_index(self.selections[index].index);
                let new_index = self.index_from_line(line);
                let length = self.line_length_from_index(new_index);
                self.selections[index].set_index_length(new_index, length);
            }

            //self.merge_overlapping_selections();
        }
    }

    fn zoom_in(&mut self) {
        if self.font_size > 6 {
            self.font_size -= 1;
            //self.check_selection_gap_up(context, self.selections.len() - 1);
            //self.check_selection_gap_down(context, self.selections.len() - 1);
        }
    }

    fn zoom_out(&mut self) {
        if self.font_size < 50 {
            self.font_size += 1;
        }
    }

    fn add_selection(&mut self, context: &Context) {
        let selection = self.selections.last().unwrap().clone();
        self.selections.push(selection);
        self.adding_selection = true;
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> Status<bool> {

        match self.dialogue_mode.clone() {

            DialogueMode::None => { },

            DialogueMode::Error(..) => self.dialogue_mode = DialogueMode::None,

            DialogueMode::OpenFile => {
                let (handled, status) = self.open_file_dialogue.handle_action(context, action);
                if let Some(completed) = status {
                    if completed {
                        let file_name = self.open_file_dialogue.file_name_box.get();
                        if let Status::Error(error) = self.open_file(file_name.clone()) { // handle the actual error
                            self.dialogue_mode = DialogueMode::Error(format_vector!("missing file {}", file_name));
                            return success!(handled);
                        }
                    }
                    self.dialogue_mode = DialogueMode::None;
                }
                return success!(handled);
            },

            DialogueMode::SetLanguage => {
                let (handled, status) = self.set_language_dialogue.handle_action(context, action);
                if let Some(completed) = status {
                    if completed && self.language != self.set_language_dialogue.language_box.get() {
                        let new_language = self.set_language_dialogue.language_box.get();
                        match Self::load_language(&new_language) {

                            Status::Success(compiler) => {
                                self.language = new_language;
                                self.compiler = compiler;
                                self.parse();
                            },

                            Status::Error(error) => { // handle the actual error
                                self.dialogue_mode = DialogueMode::Error(format_vector!("missing language file {}.data", new_language));
                                self.set_language_dialogue.language_box.clear();
                                return success!(handled);
                            }
                        }
                    }
                    self.set_language_dialogue.language_box.clear();
                    self.dialogue_mode = DialogueMode::None;
                }
                return success!(handled);
            },
        }

        match action {

            Action::CharacterMode => handle_return!(self.character_mode()),

            Action::TokenMode => handle_return!(self.token_mode()),

            Action::LineMode => handle_return!(self.line_mode()),

            Action::OpenFile => handle_return!(self.dialogue_mode = DialogueMode::OpenFile),

            Action::SetLanguage => handle_return!(self.dialogue_mode = DialogueMode::SetLanguage),

            Action::Down => handle_return!(self.move_down(context)),

            Action::Up => handle_return!(self.move_up(context)),

            Action::Left => handle_return!(self.move_left(context)),

            Action::Right => handle_return!(self.move_right(context)),

            Action::Start => handle_return!(self.move_to_start()),

            Action::End => handle_return!(self.move_to_end()),

            Action::AddSelection => handle_return!(self.add_selection(context)),

            Action::ZoomInPanel => handle_return!(self.zoom_in()),

            Action::ZoomOutPanel => handle_return!(self.zoom_out()),

            _other => return success!(false),
        }

        //match input {
            //1 => self.append(), // submode ?
            //16 => self.pop_selection(),
            //3 => self.copy_selections(),
            //22 => self.paste_selections(),
            //24 => self.cut_selections(),
            //26 => self.undo(),
            //20 => self.remove_selections(),
            //126 => self.remove_selections(),
            //10 => self.add_newline(),
            //65 => self.move_selection_up(context),
            //66 => self.move_selection_down(context),
            //67 => self.move_selection_right(context),
            //68 => self.move_selection_left(context),
            //330 => self.remove(),
            //337 => self.extend_selection_up(context),
            //336 => self.extend_selection_down(context),
            //402 => self.extend_selection_right(context),
            //393 => self.extend_selection_left(context),
        //}
    }

    pub fn add_character(&mut self, character: Character) {
        match self.dialogue_mode.clone() {

            DialogueMode::OpenFile => self.open_file_dialogue.add_character(character),

            DialogueMode::SetLanguage => self.set_language_dialogue.add_character(character),

            _other => return,
        }
    }

    fn draw_error_message(&self, framebuffer: &mut RenderTexture, context: &Context, message: &VectorString) {
        //terminal.set_color_pair(&context.theme.panel.background, &context.theme.error_color, true);
        //terminal.move_cursor(0, context.line_number_offset);
        //print!("{}", message);
        //fill_line(self.size.width - context.line_number_offset - message.len(), ' ');
    }

    fn draw_text(&self, framebuffer: &mut RenderTexture, context: &Context) {

        let mut character_text = Text::default();
        character_text.set_font(&context.font);
        character_text.set_character_size(self.font_size as u32);
        character_text.set_outline_thickness(0.0);
        character_text.set_fill_color(context.theme.panel.text);
        character_text.set_style(context.theme.panel.style);

        let character_scaling = context.character_spacing * self.font_size as f32;
        let line_scaling = context.line_spacing * self.font_size as f32;
        let line_padding = (line_scaling - self.font_size as f32) / 2.0 - (self.font_size as f32 / 7.0);
        let line_number_width = context.theme.line_number.width as f32 * character_scaling;
        let line_number_offset = match context.line_numbers {
            true => context.theme.line_number.width as f32 * character_scaling + context.theme.line_number.offset * self.font_size as f32,
            false => 0.0,
        };

        let mut token_index = 0;
        let mut draw_newline = true;
        let mut line_number = self.scroll;
        let mut index = self.index_from_line(self.scroll);
        let mut top_offset = context.theme.panel.top_offset * self.font_size as f32;
        let mut left_offset = line_number_offset + context.theme.panel.left_offset * self.font_size as f32;

        let line_number_height = line_scaling - context.theme.line_number.gap * 2.0 * line_scaling;
        let line_number_radius = context.theme.line_number.radius * line_number_height;
        let rounded = RoundedRectangle::new(context.theme.line_number.width * character_scaling, line_number_height, line_number_radius, line_number_radius, line_number_radius, line_number_radius);
        let mut line_number_base = CustomShape::new(Box::new(rounded));
        line_number_base.set_fill_color(context.theme.line_number.background);
        line_number_base.set_outline_thickness(0.0);

        let mut line_number_text = Text::default();
        line_number_text.set_font(&context.font);
        line_number_text.set_character_size(self.font_size as u32);
        line_number_text.set_outline_thickness(0.0);
        line_number_text.set_fill_color(context.theme.line_number.text);
        line_number_text.set_string(&format!("{}", line_number));
        line_number_text.set_style(context.theme.line_number.style);

        for index in index..self.content.len() {
            if top_offset >= self.size.y {
                break;
            }

            if draw_newline {
                if context.line_numbers {
                    line_number_base.set_position(Vector2f::new(context.theme.line_number.offset * self.font_size as f32, top_offset + context.theme.line_number.gap * line_scaling));
                    framebuffer.draw(&line_number_base);

                    let text_position = Vector2f::new(context.theme.line_number.offset * self.font_size as f32 + context.theme.line_number.text_offset * self.font_size as f32, top_offset + line_padding + context.theme.line_number.gap * line_scaling);
                    draw_spaced_text(framebuffer, &mut line_number_text, text_position, &format_vector!("{}", line_number), character_scaling);
                }

                draw_newline = false;
            }

            if index >= self.tokens[token_index].index + self.tokens[token_index].length {
                token_index += 1;
                if context.highlighting {
                    character_text.set_fill_color(self.tokens[token_index].get_color(context));
                }
            }

            if left_offset <= self.size.x {
                character_text.set_string(&format!("{}", self.content[index].as_char()));
                character_text.set_position(Vector2f::new(left_offset, top_offset + line_padding));
                framebuffer.draw(&character_text);
            }

            if self.content[index].is_newline() {
                left_offset = line_number_offset + context.theme.panel.left_offset * self.font_size as f32;
                top_offset += line_scaling;
                line_number += 1;
                draw_newline = true;
            } else {
                left_offset += character_scaling;
            }
        }
    }

    fn draw_selection_lines(&self, framebuffer: &mut RenderTexture, context: &Context) {

        let character_scaling = context.character_spacing * self.font_size as f32;
        let line_scaling = context.line_spacing * self.font_size as f32;
        let line_number_offset = match context.line_numbers {
            true => context.theme.line_number.width as f32 * character_scaling + context.theme.line_number.offset * self.font_size as f32,
            false => 0.0,
        };
        let mut left_offset = line_number_offset + context.theme.panel.left_offset * self.font_size as f32;

        let mut selection_line = RectangleShape::with_size(Vector2f::new(self.size.x, line_scaling));
        selection_line.set_outline_thickness(0.0);
        selection_line.set_fill_color(context.theme.selection.line);

        for index in self.selection_start()..self.selections.len() {
            let mut top_offset = self.line_from_index(self.selections[index].index) as f32 * line_scaling + context.theme.panel.top_offset * self.font_size as f32;
            let mut draw_line = true;

            for offset in 0..self.selections[index].length {
                if draw_line {
                    selection_line.set_position(Vector2f::new(left_offset, top_offset));
                    framebuffer.draw(&selection_line);
                    draw_line = false;
                }

                if self.content[self.selections[index].index].is_newline() {
                    top_offset += line_scaling;
                    draw_line = true;
                }
            }
        }
    }

    fn draw_selections(&self, framebuffer: &mut RenderTexture, context: &Context) {

        let character_scaling = context.character_spacing * self.font_size as f32;
        let line_scaling = context.line_spacing * self.font_size as f32;
        let selection_radius = context.theme.selection.radius * self.font_size as f32;
        let line_number_offset = match context.line_numbers {
            true => context.theme.line_number.width as f32 * character_scaling + context.theme.line_number.offset * self.font_size as f32,
            false => 0.0,
        };

        let rounded = RoundedRectangle::new(character_scaling, line_scaling, selection_radius, selection_radius, selection_radius, selection_radius);
        let mut single_selection_base = CustomShape::new(Box::new(rounded));
        single_selection_base.set_outline_thickness(0.0);

        let rounded = RoundedRectangle::new(character_scaling, line_scaling, selection_radius, 0.0, 0.0, selection_radius);
        let mut left_selection_base = CustomShape::new(Box::new(rounded));
        left_selection_base.set_outline_thickness(0.0);

        let rounded = RoundedRectangle::new(character_scaling, line_scaling, 0.0, selection_radius, selection_radius, 0.0);
        let mut right_selection_base = CustomShape::new(Box::new(rounded));
        right_selection_base.set_outline_thickness(0.0);

        let rounded = RoundedRectangle::new(character_scaling, line_scaling, 0.0, 0.0, 0.0, 0.0);
        let mut middle_selection_base = CustomShape::new(Box::new(rounded));
        middle_selection_base.set_outline_thickness(0.0);

        let mut selection_text = Text::default();
        selection_text.set_font(&context.font);
        selection_text.set_character_size(self.font_size as u32);
        selection_text.set_outline_thickness(0.0);
        selection_text.set_style(context.theme.selection.style);

        for (index, selection) in self.selections.iter().enumerate() {
            let mut top_offset = self.line_from_index(selection.index) as f32 * line_scaling + context.theme.panel.top_offset * self.font_size as f32;
            let mut left_offset = self.offset_from_index(selection.index) as f32 * character_scaling + line_number_offset + context.theme.panel.left_offset * self.font_size as f32;

            for offset in 0..selection.length {

                let mut base = if selection.length == 1 {
                    &mut single_selection_base
                } else if offset == 0 {
                    &mut left_selection_base
                } else if offset == selection.length - 1 {
                    &mut right_selection_base
                } else {
                    &mut middle_selection_base
                };

                if self.adding_selection && index == self.selections.len() - 1 {
                    base.set_fill_color(context.theme.selection.new_background);
                    selection_text.set_fill_color(context.theme.selection.new_text);
                } else {
                    base.set_fill_color(context.theme.selection.background);
                    selection_text.set_fill_color(context.theme.selection.text);
                }

                base.set_position(Vector2f::new(left_offset, top_offset));
                selection_text.set_position(Vector2f::new(left_offset, top_offset));
                selection_text.set_string(&format!("{}", self.content[selection.index + offset]));

                framebuffer.draw(base);
                framebuffer.draw(&selection_text);

                if self.content[self.selections[index].index].is_newline() {
                    left_offset = line_number_offset + context.theme.panel.left_offset * self.font_size as f32;
                    top_offset += line_scaling;
                } else {
                    left_offset += character_scaling;
                }
            }
        }
    }

    fn draw_status_bar(&self, framebuffer: &mut RenderTexture, context: &Context) {

        if context.status_bar {
            let status_bar_height = context.theme.status_bar.height * context.font_size as f32;
            let mut status_bar = RectangleShape::with_size(Vector2f::new(self.size.x, status_bar_height));
            status_bar.set_fill_color(context.theme.status_bar.background);
            status_bar.set_outline_thickness(0.0);
            status_bar.set_position(Vector2f::new(0.0, self.size.y - status_bar_height));
            framebuffer.draw(&status_bar);
        } else {
            let focus_bar_height = context.theme.focus_bar.height * context.font_size as f32;
            let mut focus_bar = RectangleShape::with_size(Vector2f::new(self.size.x, focus_bar_height));
            focus_bar.set_fill_color(context.theme.focus_bar.background);
            focus_bar.set_outline_thickness(0.0);
            focus_bar.set_position(Vector2f::new(0.0, self.size.y - focus_bar_height));
            framebuffer.draw(&focus_bar);
        }

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

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, focused: bool) {

        framebuffer.clear(context.theme.panel.background);

        if context.selection_lines && self.dialogue_mode.selections_shown() {
            self.draw_selection_lines(framebuffer, context);
        }

        self.draw_text(framebuffer, context);

        if self.dialogue_mode.selections_shown() {
            self.draw_selections(framebuffer, context);
        }

        match &self.dialogue_mode {
            DialogueMode::None => { },
            DialogueMode::Error(message) => self.draw_error_message(framebuffer, context, message),
            DialogueMode::OpenFile => self.open_file_dialogue.draw(framebuffer, context, self.size.x),
            DialogueMode::SetLanguage => self.set_language_dialogue.draw(framebuffer, context, self.size.x),
        }

        if focused {
            self.draw_status_bar(framebuffer, context);
        }

        framebuffer.display();
    }
}
