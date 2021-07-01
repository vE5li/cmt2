use seamonkey::*;
use super::super::Selection;
use context::{ Context, Action };
use sfml::system::Vector2f;
use sfml::graphics::*;
use graphics::{ RoundedRectangle, draw_spaced_text };

macro_rules! handle_return_none {
    ($expression: expr) => ({
        $expression;
        return (true, None);
    })
}

pub struct TextBox {
    pub description: SharedString,
    pub content: SharedString,
    pub selection: Selection,
    pub displacement: usize,
}

impl TextBox {

    pub fn new(description: &'static str, displacement: usize) -> Self {
        Self {
            description: SharedString::from(description),
            content: SharedString::from(" "),
            selection: Selection::new(0, 0, 0),
            displacement: displacement,
        }
    }

    pub fn from(description: &'static str, displacement: usize, content: SharedString) -> Self {
        Self {
            description: SharedString::from(description),
            content: format_shared!("{} ", content),
            selection: Selection::new(content.len(), content.len(), 0),
            displacement: displacement,
        }
    }

    pub fn get(&self) -> SharedString {
        let mut cut_content = self.content.clone();
        cut_content.pop();
        return cut_content;
    }

    pub fn set(&mut self, content: SharedString) {
        self.selection.primary_index = 0;
        self.selection.secondary_index = 0;
        self.content = format_shared!("{} ", content);
    }

    fn remove_character(&mut self) {
        //if self.selection.index > 0 && self.content.len() > 1 {

        //    // TODO: delete selected text if necessary
        //    self.selection.reset();
        //    self.selection.index -= 1;
        //    self.content.remove(self.selection.index);
        //}
    }

    fn delete_character(&mut self) {
        //if self.selection.index < self.content.len() - 1 {
        //    self.content.remove(self.selection.index);
        //}
    }

    pub fn clear(&mut self) {
        //self.content = SharedString::from(" ");
        //self.selection.reset();
        //self.selection.index = 0;
    }

    pub fn add_character(&mut self, character: Character) {

        // TODO: delete selected text if necessary
        //self.selection.reset();

        //self.content.insert(self.selection.index, character);
        //self.selection.index += 1;
    }

    pub fn handle_action(&mut self, action: Action) -> (bool, Option<bool>) {
        match action {

            Action::Abort => return (true, Some(false)),

            Action::Confirm => return (true, Some(true)),

            Action::Left => handle_return_none!(self.move_left()),

            Action::Right => handle_return_none!(self.move_right()),

            Action::Remove => handle_return_none!(self.remove_character()),

            Action::Delete => handle_return_none!(self.delete_character()),

            Action::DeleteLine => handle_return_none!(self.clear()),

            _other => return (false, None),
        }
    }

    fn move_left(&mut self) {
        //if self.selection.index > 0 {
        //    self.selection.reset();
        //    self.selection.index -= 1;
        //}
    }

    fn move_right(&mut self) {
        //if self.selection.index < self.content.len() - 1 {
        //    self.selection.reset();
        //    self.selection.index += 1;
        //}
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, width: f32, offset: Vector2f, focused: bool) {

        let character_scaling = context.character_spacing * context.font_size as f32;
        let dialogue_height = context.theme.dialogue.height * context.font_size as f32;
        let corner_radius = context.theme.dialogue.corner_radius * dialogue_height;

        let rounded = RoundedRectangle::new(width, dialogue_height, corner_radius, corner_radius, corner_radius, corner_radius);
        let mut text_box_base = CustomShape::new(Box::new(rounded));
        text_box_base.set_outline_thickness(0.0);

        let mut text_box_text = Text::default();
        text_box_text.set_font(&context.font);
        text_box_text.set_character_size(context.font_size as u32);
        text_box_text.set_outline_thickness(0.0);

        text_box_base.set_fill_color(context.theme.dialogue.background);
        text_box_text.set_fill_color(context.theme.dialogue.text);

        match self.content.len() > 1 {
            true => text_box_text.set_style(context.theme.dialogue.text_style),
            false => text_box_text.set_style(context.theme.dialogue.ghost_style),
        }

        if focused {
            text_box_base.set_fill_color(context.theme.dialogue.focused);
            match self.content.len() > 1 {
                true => text_box_text.set_fill_color(context.theme.dialogue.focused_text),
                false => text_box_text.set_fill_color(context.theme.dialogue.focused_ghost),
            }
        } else {
            text_box_base.set_fill_color(context.theme.dialogue.background);
            match self.content.len() > 1 {
                true => text_box_text.set_fill_color(context.theme.dialogue.text),
                false => text_box_text.set_fill_color(context.theme.dialogue.ghost),
            }
        }

        let text_position = Vector2f::new(offset.x + context.theme.dialogue.text_offset * character_scaling, offset.y);
        text_box_base.set_position(offset);
        framebuffer.draw(&text_box_base);

        match self.content.len() > 1 {
            true => draw_spaced_text(framebuffer, &mut text_box_text, text_position, &self.content, character_scaling),
            false => draw_spaced_text(framebuffer, &mut text_box_text, text_position, &self.description, character_scaling),
        }
    }
}
