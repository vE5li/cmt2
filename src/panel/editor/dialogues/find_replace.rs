use kami::*;
use context::{ Context, Action };
use sfml::graphics::*;
use sfml::system::Vector2f;
use super::super::TextBox;

pub struct FindReplaceDialogue {
    pub find_box: TextBox,
    pub replace_box: TextBox,
    find_focused: bool,
}

impl FindReplaceDialogue {

    pub fn new() -> Self {
        Self {
            find_box: TextBox::new("find", " > ", 0),
            replace_box: TextBox::new("replace", " > ", 1),
            find_focused: true,
        }
    }

    pub fn handle_action(&mut self, action: Action) -> (bool, Option<bool>) {

        if let Action::FindReplace = action {
            return (true, Some(false));
        }

        if let Action::FocusNext = action {
            self.find_focused = !self.find_focused;
            return (true, None);
        }

        if let Action::Up = action {
            self.find_focused = true;
            return (true, None);
        }

        if let Action::Down = action {
            self.find_focused = false;
            return (true, None);
        }

        if self.find_focused {
            let (handled, status) = self.find_box.handle_action(action);
            if let Some(completed) = status {
                if completed {
                    self.find_focused = false;
                    return (true, None);
                }
            }

            return (handled, status);
        }


        let (handled, status) = self.replace_box.handle_action(action);
        if let Some(completed) = status {
            if completed && self.find_box.get().is_empty() {
                self.find_focused = true;
                return (true, None);
            }
        }

        return (handled, status);
    }

    pub fn add_character(&mut self, character: Character) {
        match self.find_focused {
            true => self.find_box.add_character(character),
            false => self.replace_box.add_character(character),
        }
    }

    pub fn reset(&mut self) {
        self.find_box.clear();
        self.replace_box.clear();
        self.find_focused = true;
    }

    pub fn draw(&self, framebuffer: &mut RenderTexture, context: &Context, size: Vector2f, offset: Vector2f) {
        let dialogue_height = context.theme.dialogue.height * context.font_size as f32;
        let replace_offset = Vector2f::new(offset.x, offset.y + dialogue_height);

        self.find_box.draw(framebuffer, context, size.x, offset, self.find_focused);
        self.replace_box.draw(framebuffer, context, size.x, replace_offset, !self.find_focused);
    }
}
