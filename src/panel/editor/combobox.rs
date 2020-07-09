use kami::*;
use super::{ TextBox, fill_line };
use context::{ Context, Action };
use sfml::graphics::*;

macro_rules! handle_return_none {
    ($expression: expr) => ({
        $expression;
        return (true, None);
    })
}

macro_rules! handle_maybe_return {
    ($expression: expr) => ({
        if $expression {
            return (true, None);
        }
    })
}

#[derive(Clone)]
pub enum ComboSelection {
    TextBox,
    Variant(usize, VectorString),
}

impl ComboSelection {

    pub fn is_textbox(&self) -> bool {
        match self {
            ComboSelection::TextBox => return true,
            _other => return false,
        }
    }

    pub fn index_matches(&self, selected: usize) -> bool {
        match self {
            ComboSelection::TextBox => return false,
            ComboSelection::Variant(index, _original) => return *index == selected,
        }
    }
}

pub struct ComboBox {
    pub textbox: TextBox,
    pub allow_unknown: bool,
    pub variants: Vec<VectorString>,
    pub selection: ComboSelection,
    pub displacement: usize,
    pub path_mode: bool,
    pub scroll: usize,
}

impl ComboBox {

    pub fn new(description: &'static str, prompt: &'static str, displacement: usize, allow_unknown: bool, path_mode: bool, variants: Vec<VectorString>) -> Self {
        Self {
            textbox: TextBox::new(description, prompt, displacement),
            allow_unknown: allow_unknown,
            variants: variants,
            selection: ComboSelection::TextBox,
            displacement: displacement,
            path_mode: path_mode,
            scroll: 0,
        }
    }

    fn move_up(&mut self, context: &Context) {
        if let ComboSelection::Variant(index, original) = self.selection.clone() {
            if index == 0 {
                self.selection = ComboSelection::TextBox;
                self.textbox.set(original);
            } else {
                let new_index = index - 1;
                let valid_variants = self.valid_variants();
                self.selection = ComboSelection::Variant(new_index, original.clone());

                match self.path_mode {
                    true => self.textbox.set(self.get_combined(&valid_variants[new_index])),
                    false => self.textbox.set(valid_variants[new_index].clone())
                }

                if self.scroll != 0 && context.selection_gap + self.scroll > new_index {
                    self.scroll -= 1;
                }
            }
        }
    }

    fn move_down(&mut self, context: &Context) {
        if let ComboSelection::TextBox = self.selection.clone() {
            let valid_variants = self.valid_variants();
            if !valid_variants.is_empty() {
                self.selection = ComboSelection::Variant(0, self.textbox.get());

                match self.path_mode {
                    true => self.textbox.set(self.get_combined(&valid_variants[0])),
                    false => self.textbox.set(valid_variants[0].clone())
                }
            }
            return;
        }

        if let ComboSelection::Variant(index, original) = self.selection.clone() {
            let valid_variants = self.valid_variants();

            if index < valid_variants.len() - 1 {
                self.selection = ComboSelection::Variant(index + 1, original.clone());
                self.textbox.set(valid_variants[index + 1].clone());

                match self.path_mode {
                    true => self.textbox.set(self.get_combined(&valid_variants[index + 1])),
                    false => self.textbox.set(valid_variants[index + 1].clone())
                }

                if context.selection_gap + self.displacement + 4 > context.height + self.scroll - index {
                    self.scroll += 1;
                }
            }
        }
    }

    pub fn get_combined(&self, suffix: &VectorString) -> VectorString {
        let original = match &self.selection {
            ComboSelection::Variant(_index, original) => original.clone(),
            ComboSelection::TextBox => self.textbox.get(),
        };

        let positions = original.position(&VectorString::from("/"));
        if !positions.is_empty() {
            let mut combined = original.slice(0, positions[positions.len() - 1]);
            combined.push_str(suffix);
            return combined;
        }

        return suffix.clone();
    }

    pub fn valid_variants(&self) -> Vec<VectorString> {
        let mut original = match &self.selection {
            ComboSelection::Variant(_index, original) => original.clone(),
            ComboSelection::TextBox => self.textbox.get(),
        };

        if self.path_mode {
            let pieces = original.split(&VectorString::from("/"), false);
            original = pieces[pieces.len() - 1].clone();
        }

        let mut valid_variants = self.variants.clone();
        valid_variants.retain(|variant| variant.contains(&original));
        return valid_variants;
    }

    pub fn get(&self) -> VectorString {
        return self.textbox.get();
    }

    pub fn clear(&mut self) {
        self.reset_selection();
        self.textbox.clear();
    }

    fn reset_selection(&mut self) {
        self.selection = ComboSelection::TextBox;
        self.scroll = 0;
    }

    fn handle_confirm(&mut self) -> bool {
        if !self.allow_unknown && self.selection.is_textbox() {
            let valid_variants = self.valid_variants();
            if valid_variants.is_empty() {
                return true
            }

            match self.path_mode {
                true => self.textbox.set(self.get_combined(&valid_variants[0])),
                false => self.textbox.set(valid_variants[0].clone()),
            }

            return self.path_mode && *self.textbox.get().chars().last().unwrap() == Character::from_char('/');
        }
        return false;
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> (bool, Option<bool>) {
        match action {

            Action::Up => handle_return_none!(self.move_up(context)),

            Action::Down => handle_return_none!(self.move_down(context)),

            Action::Left => self.reset_selection(),

            Action::Right => self.reset_selection(),

            Action::Start => self.reset_selection(),

            Action::End => self.reset_selection(),

            Action::Remove => self.reset_selection(),

            Action::Delete => self.reset_selection(),

            Action::Clear => self.reset_selection(),

            Action::ExtendLeft => self.reset_selection(),

            Action::ExtendRight => self.reset_selection(),

            Action::MoveLeft => self.reset_selection(),

            Action::MoveRight => self.reset_selection(),

            Action::Copy => self.reset_selection(),

            Action::Paste => self.reset_selection(),

            Action::Cut => self.reset_selection(),

            Action::Confirm => handle_maybe_return!(self.handle_confirm()),

            _other => { },
        }

        return self.textbox.handle_action(action);
    }

    pub fn add_character(&mut self, character: Character) {
        self.reset_selection();
        self.textbox.add_character(character);
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, width: f32, offset: usize, focused: bool) {
        self.textbox.draw(framebuffer, context, width, offset, focused && self.selection.is_textbox());

        //if focused {
        //    let mut top_offset = self.displacement + 2;
        //    let valid_variants = self.valid_variants();
        //    for index in self.scroll..valid_variants.len() {

        //        if top_offset >= context.height {
        //            return;
        //        }

        //        match self.selection.index_matches(index) {
        //            true => terminal.set_color_pair(&context.theme.panel_color, &context.theme.input_focused_color, true),
        //            false => terminal.set_color_pair(&context.theme.panel_color, &context.theme.input_color, true),
        //        }

        //        terminal.move_cursor(top_offset, offset + context.line_number_offset);
        //        fill_line(width - context.line_number_offset - 1, ' ');
        //        terminal.move_cursor(top_offset, offset + context.line_number_offset + self.textbox.prompt.len());
        //        print!("{}", valid_variants[index]);
        //        top_offset += 1;
        //    }
        //}
    }
}
