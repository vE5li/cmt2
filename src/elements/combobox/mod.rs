mod selection;
mod item;

use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use themes::{ InterfaceTheme, DialogueTheme, ItemTheme };
use elements::{ TextBox, Textfield };
use dialogues::DialogueStatus;
use managers::LanguageManager;
use interface::InterfaceContext;
use system::subtract_or_zero;

pub use self::selection::ComboSelection;
pub use self::item::ComboItem;

macro_rules! handle_return_none {
    ($expression: expr) => ({
        $expression;
        return DialogueStatus::handled();
    })
}

macro_rules! handle_maybe_return_none {
    ($expression: expr) => ({
        if ($expression) {
            return DialogueStatus::handled();
        }
    })
}

pub struct ComboBox<I: ComboItem + Clone> {
    textbox: TextBox,
    allow_unknown: bool,
    items: Vec<I>,
    selection: ComboSelection,
    displacement: usize,
    scroll: usize,
    size: Vector2f,
    position: Vector2f,
    line_count: usize,
}

impl<I: ComboItem + Clone> ComboBox<I> {

    pub fn new(language_manager: &mut LanguageManager, description: &'static str, displacement: usize, allow_unknown: bool, items: Vec<I>) -> Self {
        Self {
            textbox: TextBox::new(language_manager, description, displacement),
            allow_unknown: allow_unknown,
            items: items,
            selection: ComboSelection::TextBox,
            displacement: displacement,
            scroll: 0,
            size: Vector2f::new(0., 0.),
            position: Vector2f::new(0., 0.),
            line_count: 0,
        }
    }

    fn move_up(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager) {
        if let ComboSelection::Item(index, original) = self.selection.clone() {
            if index == 0 {
                self.selection = ComboSelection::TextBox;
                self.textbox.set_text_without_save(language_manager, original);
            } else {
                let new_index = index - 1;
                let valid_items = self.valid_items();
                self.selection = ComboSelection::Item(new_index, original.clone());

                let text = valid_items[new_index].update_name();
                self.textbox.set_text_without_save(language_manager, text);
                self.check_selection_gaps(interface_context, new_index);
            }
        }
    }

    fn move_down(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager) {
        if let ComboSelection::TextBox = self.selection.clone() {
            let valid_items = self.valid_items();

            if !valid_items.is_empty() {
                self.selection = ComboSelection::Item(0, self.textbox.get_text());
                let text = valid_items[0].update_name();
                self.textbox.set_text_without_save(language_manager, text);
            }

            return;
        }

        if let ComboSelection::Item(index, original) = self.selection.clone() {
            let valid_items = self.valid_items();

            if index + 1 < valid_items.len() {
                self.selection = ComboSelection::Item(index + 1, original);
                let text = valid_items[index + 1].update_name();
                self.textbox.set_text_without_save(language_manager, text);
                self.check_selection_gaps(interface_context, index + 1);
            }
        }
    }

    fn check_selection_gaps(&mut self, interface_context: &InterfaceContext, index: usize) {
        if interface_context.selection_gap * 2 >= self.line_count {
            self.scroll = subtract_or_zero(index, self.line_count / 2);
        } else if interface_context.selection_gap + self.scroll > index {
            self.scroll = subtract_or_zero(index, interface_context.selection_gap);
        } else if index + 1 - self.scroll + interface_context.selection_gap > self.line_count {
            self.scroll = index + 1 - (self.line_count - interface_context.selection_gap);
        }
    }

    pub fn set_items(&mut self, items: Vec<I>) {
        self.items = items;
        // cap selection !!!!!!
    }

    pub fn valid_items(&self) -> Vec<I> {
        let mut valid_items = self.items.clone();
        let original = match &self.selection {
            ComboSelection::Item(_index, original) => original.clone(),
            ComboSelection::TextBox => self.textbox.get_text(),
        };
        valid_items.retain(|item| item.update_name().contains(&original));
        return valid_items;
    }

    pub fn remove_selected_item(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager) {
        if let ComboSelection::Item(index, original) = &self.selection {
            self.items.remove(*index);
        }

        if let ComboSelection::Item(index, original) = self.selection.clone() {
            let valid_items = self.valid_items();

            if valid_items.is_empty() {
                self.selection = ComboSelection::TextBox;
                self.textbox.set_text_without_save(language_manager, original);
                return;
            }

            let new_index = match index >= valid_items.len() {
                true => index - 1,
                false => index,
            };

            self.selection = ComboSelection::Item(new_index, original.clone());
            let text = valid_items[new_index].update_name();
            self.textbox.set_text_without_save(language_manager, text);
            self.check_selection_gaps(interface_context, new_index);
        }
    }

    pub fn get_text(&self) -> SharedString {
        return self.textbox.get_text();
    }

    pub fn get_value(&self) -> I::Value {
        let text = self.get_text();
        return self.items.iter().find(|item| item.update_name() == text).unwrap().return_value();
    }

    pub fn get_selection(&self) -> ComboSelection {
        return self.selection.clone();
    }

    pub fn get_original(&self) -> SharedString {
        match &self.selection {
           ComboSelection::Item(_index, original) => return original.clone(),
           ComboSelection::TextBox => return self.textbox.get_text(),
        }
    }

    pub fn is_textbox_focused(&self) -> bool {
        return self.selection.is_textbox();
    }

    pub fn set_text(&mut self, language_manager: &mut LanguageManager, text: SharedString) {
        self.textbox.set_text(language_manager, text);
    }

    pub fn clear(&mut self, language_manager: &mut LanguageManager) {
        self.reset_selection();
        self.textbox.clear(language_manager);
    }

    pub fn reset_selection(&mut self) {
        self.selection = ComboSelection::TextBox;
        self.scroll = 0;
    }

    fn focus_next(&mut self, language_manager: &mut LanguageManager) -> bool {
        let valid_items = self.valid_items();
        if valid_items.is_empty() {
            return false;
        }

        let suffix = match &self.selection {
            ComboSelection::Item(index, _original) => valid_items[*index].update_name(),
            ComboSelection::TextBox => valid_items[0].update_name(),
        };

        self.textbox.set_text(language_manager, suffix);
        self.reset_selection();
        return true;
    }

    fn handle_confirm(&mut self, language_manager: &mut LanguageManager) -> DialogueStatus {
        if !self.allow_unknown && self.selection.is_textbox() {
            let valid_items = self.valid_items();

            if valid_items.is_empty() {
                return DialogueStatus::handled();
            }

            self.textbox.set_text(language_manager, valid_items[0].update_name());
        }
        return DialogueStatus::completed();
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {

            Action::Up => handle_return_none!(self.move_up(interface_context, language_manager)),

            Action::Down => handle_return_none!(self.move_down(interface_context, language_manager)),

            Action::FocusNext => handle_maybe_return_none!(self.focus_next(language_manager)),

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

            Action::Undo => self.reset_selection(),

            Action::Redo => self.reset_selection(),

            _other => { },
        }

        if let Some(action) = self.textbox.handle_action(language_manager, action) {
            match action {

                Action::Confirm => return self.handle_confirm(language_manager),

                Action::Abort => return DialogueStatus::aborted(),

                _unhandled => return DialogueStatus::unhandled(),
            }
        }

        return DialogueStatus::handled();
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.reset_selection();
        self.textbox.add_character(language_manager, character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.textbox.update_layout(interface_context, theme, size, position);

        let float_font_size = interface_context.font_size as f32;
        let height = (size.y - theme.height * float_font_size) * theme.display_height;
        let element_height = (theme.height * float_font_size) + (theme.unfocused_item_theme.padding * float_font_size);

        self.line_count = (height / element_height) as usize;
        self.size = size;
        self.position = position;

        if let ComboSelection::Item(index, ..) = self.selection.clone() {
            self.check_selection_gaps(interface_context, index);
        }
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme, focused: bool) {
        self.textbox.render(framebuffer, interface_context, theme, focused && self.selection.is_textbox());

        if focused {

            let padding = match self.selection.is_textbox() {
                true => theme.focused_textbox_theme.padding * interface_context.font_size as f32,
                false => theme.unfocused_textbox_theme.padding * interface_context.font_size as f32,
            };

            let dialogue_height = theme.height * interface_context.font_size as f32;
            let mut top_position = self.position.y + padding + (self.displacement + 1) as f32 * dialogue_height;
            let size = Vector2f::new(self.size.x, dialogue_height);
            let valid_items = self.valid_items();

            for index in self.scroll..valid_items.len() {
                if top_position > self.size.y || index - self.scroll >= self.line_count {
                    break;
                }

                let item_theme = match self.selection.index_matches(index) {
                    true => &theme.focused_item_theme,
                    false => &theme.unfocused_item_theme,
                };

                let position = Vector2f::new(self.position.x, top_position);
                Textfield::render(framebuffer, interface_context, valid_items[index].display_theme(item_theme), &valid_items[index].display_name(), size, position, dialogue_height);
                top_position += dialogue_height + item_theme.padding * interface_context.font_size as f32;
            }
        }
    }
}
