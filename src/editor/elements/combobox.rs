use seamonkey::*;
use super::TextBox;
use context::{ Context, Action };
use sfml::graphics::*;
use sfml::system::Vector2f;
use graphics::{ RoundedRectangle, draw_spaced_text };

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
    Variant(usize, SharedString),
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
    pub variants: Vec<SharedString>,
    pub selection: ComboSelection,
    pub displacement: usize,
    pub path_mode: bool,
    pub scroll: usize,
}

impl ComboBox {

    pub fn new(description: &'static str, displacement: usize, allow_unknown: bool, path_mode: bool, variants: Vec<SharedString>) -> Self {
        Self {
            textbox: TextBox::new(description, displacement),
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

                // UNCOMMENT!!!!
                //if context.selection_gap + self.displacement + 4.0 > context.height + self.scroll - index {
                //    self.scroll += 1;
                //}
            }
        }
    }

    pub fn get_combined(&self, suffix: &SharedString) -> SharedString {
        let original = match &self.selection {
            ComboSelection::Variant(_index, original) => original.clone(),
            ComboSelection::TextBox => self.textbox.get(),
        };

        let positions = original.position(&SharedString::from("/"));
        if !positions.is_empty() {
            let mut combined = original.slice(0, positions[positions.len() - 1]);
            combined.push_str(suffix);
            return combined;
        }

        return suffix.clone();
    }

    pub fn valid_variants(&self) -> Vec<SharedString> {
        let mut original = match &self.selection {
            ComboSelection::Variant(_index, original) => original.clone(),
            ComboSelection::TextBox => self.textbox.get(),
        };

        if self.path_mode {
            let pieces = original.split(&SharedString::from("/"), false);
            original = pieces[pieces.len() - 1].clone();
        }

        let mut valid_variants = self.variants.clone();
        valid_variants.retain(|variant| variant.contains(&original));
        return valid_variants;
    }

    pub fn get(&self) -> SharedString {
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

            Action::DeleteLine => self.reset_selection(),

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

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, size: Vector2f, offset: Vector2f, focused: bool) {
        self.textbox.draw(framebuffer, context, size.x, offset, focused && self.selection.is_textbox());

        if focused {

            let character_scaling = context.character_spacing * context.font_size as f32;
            let dialogue_height = context.theme.dialogue.height * context.font_size as f32;
            let corner_radius = context.theme.dialogue.corner_radius;

            let rounded = RoundedRectangle::new(size.x, dialogue_height, corner_radius, corner_radius, corner_radius, corner_radius);
            let mut text_box_base = CustomShape::new(Box::new(rounded));
            text_box_base.set_outline_thickness(0.0);

            let mut text_box_text = Text::default();
            text_box_text.set_font(&context.font);
            text_box_text.set_character_size(context.font_size as u32);
            text_box_text.set_outline_thickness(0.0);
            text_box_text.set_style(context.theme.dialogue.text_style);

            let mut top_offset = offset.y + (self.displacement + 1) as f32 * dialogue_height;
            let valid_variants = self.valid_variants();
            for index in 0..valid_variants.len() {

                if top_offset > size.y {
                    break;
                }

                if self.selection.index_matches(index) {
                    text_box_base.set_fill_color(context.theme.dialogue.focused);
                    text_box_text.set_fill_color(context.theme.dialogue.focused_text);
                } else {
                    text_box_base.set_fill_color(context.theme.dialogue.background);
                    text_box_text.set_fill_color(context.theme.dialogue.text);
                }

                text_box_base.set_position(Vector2f::new(offset.x, top_offset));
                framebuffer.draw(&text_box_base);

                draw_spaced_text(framebuffer, &mut text_box_text, Vector2f::new(offset.x + context.theme.dialogue.text_offset * character_scaling, top_offset), &valid_variants[index], character_scaling);
                top_offset += dialogue_height;
            }
        }
    }
}
