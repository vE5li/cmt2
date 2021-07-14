use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use system::LanguageManager;
use interface::InterfaceContext;
use dialogues::DialogueTheme;
use elements::TextBox;
use input::Action;

pub struct FindReplaceDialogue {
    find_textbox: TextBox,
    replace_textbox: TextBox,
    find_focused: bool,
}

impl FindReplaceDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {
        Self {
            find_textbox: TextBox::new(language_manager, "find", 0),
            replace_textbox: TextBox::new(language_manager, "replace", 1),
            find_focused: true,
        }
    }

    pub fn handle_action(&mut self, language_manager: &mut LanguageManager, action: Action) -> (bool, Option<bool>) {

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

        /*if self.find_focused {
            let (handled, status) = self.find_textbox.handle_action(language_manager, action);

            if let Some(completed) = status {
                if completed {
                    self.find_focused = false;
                    return (true, None);
                }
            }

            return (handled, status);
        }


        let (handled, status) = self.replace_textbox.handle_action(language_manager, action);
        if let Some(completed) = status {
            if completed && self.find_textbox.get().is_empty() {
                self.find_focused = true;
                return (true, None);
            }
        }*/

        //return (handled, status);
        return (true, None);
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        match self.find_focused {
            true => self.find_textbox.add_character(language_manager, character),
            false => self.replace_textbox.add_character(language_manager, character),
        }
    }

    pub fn reset(&mut self, language_manager: &mut LanguageManager) {
        self.find_textbox.clear(language_manager);
        self.replace_textbox.clear(language_manager);
        self.find_focused = true;
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        let dialogue_height = theme.height * interface_context.font_size as f32;
        let replace_position = Vector2f::new(position.x, position.y + dialogue_height);

        self.find_textbox.update_layout(interface_context, theme, size, position);
        self.replace_textbox.update_layout(interface_context, theme, size, replace_position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.find_textbox.render(framebuffer, interface_context, theme, self.find_focused);
        self.replace_textbox.render(framebuffer, interface_context, theme, !self.find_focused);
    }
}
