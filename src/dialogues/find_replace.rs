use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use interface::InterfaceContext;
use dialogues::DialogueTheme;
use elements::TextBox;
use input::Action;

pub struct FindReplaceDialogue {
    pub find_box: TextBox,
    pub replace_box: TextBox,
    find_focused: bool,
}

impl FindReplaceDialogue {

    pub fn new() -> Self {
        Self {
            find_box: TextBox::new("find", 0),
            replace_box: TextBox::new("replace", 1),
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

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        let dialogue_height = theme.height * interface_context.font_size as f32;
        let replace_position = Vector2f::new(position.x, position.y + dialogue_height);

        self.find_box.update_layout(interface_context, theme, size, position);
        self.replace_box.update_layout(interface_context, theme, size, replace_position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.find_box.render(framebuffer, interface_context, theme, self.find_focused);
        self.replace_box.render(framebuffer, interface_context, theme, !self.find_focused);
    }
}
