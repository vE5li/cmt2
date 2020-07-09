use kami::*;
use super::{ Selection, fill_line };
use context::{ Context, Action };
use sfml::system::Vector2f;
use sfml::graphics::*;
use graphics::draw_spaced_text;

macro_rules! handle_return_none {
    ($expression: expr) => ({
        $expression;
        return (true, None);
    })
}

pub struct TextBox {
    pub description: VectorString,
    pub prompt: VectorString,
    pub content: VectorString,
    pub selection: Selection,
    pub displacement: usize,
}

impl TextBox {

    pub fn new(description: &'static str, prompt: &'static str, displacement: usize) -> Self {
        Self {
            description: VectorString::from(description),
            prompt: VectorString::from(prompt),
            content: VectorString::from(" "),
            selection: Selection::new(0, 1, 0),
            displacement: displacement,
        }
    }

    pub fn from(description: &'static str, prompt: &'static str, displacement: usize, content: VectorString) -> Self {
        Self {
            description: VectorString::from(description),
            prompt: VectorString::from(prompt),
            content: format_vector!("{} ", content),
            selection: Selection::new(content.len(), 1, 0),
            displacement: displacement,
        }
    }

    pub fn get(&self) -> VectorString {
        let mut cut_content = self.content.clone();
        cut_content.pop();
        return cut_content;
    }

    pub fn set(&mut self, content: VectorString) {
        self.selection.reset();
        self.selection.index = content.len();
        self.content = format_vector!("{} ", content);
    }

    fn remove_character(&mut self) {
        if self.selection.index > 0 && self.content.len() > 1 {

            // TODO: delete selected text if necessary
            self.selection.reset();
            self.selection.index -= 1;
            self.content.remove(self.selection.index);
        }
    }

    fn delete_character(&mut self) {
        if self.selection.index < self.content.len() - 1 {
            self.content.remove(self.selection.index);
        }
    }

    pub fn clear(&mut self) {
        self.content = VectorString::from(" ");
        self.selection.reset();
        self.selection.index = 0;
    }

    pub fn add_character(&mut self, character: Character) {

        // TODO: delete selected text if necessary
        self.selection.reset();

        self.content.insert(self.selection.index, character);
        self.selection.index += 1;
    }

    pub fn handle_action(&mut self, action: Action) -> (bool, Option<bool>) {
        match action {

            Action::Abort => return (true, Some(false)),

            Action::Confirm => return (true, Some(true)),

            Action::Left => handle_return_none!(self.move_left()),

            Action::Right => handle_return_none!(self.move_right()),

            Action::Remove => handle_return_none!(self.remove_character()),

            Action::Delete => handle_return_none!(self.delete_character()),

            Action::Clear => handle_return_none!(self.clear()),

            _other => return (false, None),
        }
    }

    fn move_left(&mut self) {
        if self.selection.index > 0 {
            self.selection.reset();
            self.selection.index -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.selection.index < self.content.len() - 1 {
            self.selection.reset();
            self.selection.index += 1;
        }
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, width: f32, offset: usize, focused: bool) {

        let dialogue_height = context.theme.dialogue.height * context.font_size as f32;
        let mut text_box_base = RectangleShape::with_size(Vector2f::new(width, dialogue_height));
        text_box_base.set_outline_thickness(0.0);

        let mut text_box_text = Text::default();
        text_box_text.set_font(&context.font);
        text_box_text.set_character_size(context.font_size as u32);
        text_box_text.set_outline_thickness(0.0);
        text_box_text.set_style(context.theme.dialogue.style);

        text_box_base.set_fill_color(context.theme.dialogue.background);
        text_box_text.set_fill_color(context.theme.dialogue.text);

        let character_scaling = context.character_spacing * context.font_size as f32;
        let center = (width - self.description.len() as f32 * character_scaling) / 2.0;
        text_box_base.set_position(Vector2f::new(0.0, offset as f32 * dialogue_height));
        framebuffer.draw(&text_box_base);

        draw_spaced_text(framebuffer, &mut text_box_text, Vector2f::new(center, offset as f32 * dialogue_height), &self.description, character_scaling);

        if focused {
            text_box_base.set_fill_color(context.theme.dialogue.focused);
            text_box_text.set_fill_color(context.theme.dialogue.focused_text);
        } else {
            text_box_base.set_fill_color(context.theme.dialogue.background);
            text_box_text.set_fill_color(context.theme.dialogue.text);
        }

        text_box_base.set_position(Vector2f::new(0.0, (offset + 1) as f32 * dialogue_height));
        framebuffer.draw(&text_box_base);

        draw_spaced_text(framebuffer, &mut text_box_text, Vector2f::new(0.0, (offset + 1) as f32 * dialogue_height), &self.content, character_scaling);

        //text_box_text.set_position();
        //text_box_text.set_string(&self.content.printable());
        //framebuffer.draw(&text_box_text);

        //let center = (width - context.line_number_offset - self.description.len()) / 2;
        //terminal.move_cursor(self.displacement, offset + context.line_number_offset);
        //fill_line(width - context.line_number_offset - 1, ' ');
        //terminal.move_cursor(self.displacement, offset + center);
        //print!("{}", self.description);

        //terminal.move_cursor(self.displacement + 1, offset + context.line_number_offset);
        //print!("{}{}", self.prompt, self.content);
        //fill_line(width - context.line_number_offset - self.content.len() - self.prompt.len() - 1, ' ');

        //if focused {
        //    let offset = offset + context.line_number_offset + self.selection.index + self.prompt.len();
        //    terminal.set_color_pair(&context.theme.panel_color, &context.theme.selection_color, true);
        //    terminal.move_cursor(self.displacement + 1, offset);
        //    print!("{}", self.content[self.selection.index].as_char());
        //}
    }
}
