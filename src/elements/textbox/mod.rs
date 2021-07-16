mod theme;

use seamonkey::*;

use dialogues::DialogueTheme;
use elements::{ Textbuffer, TextbufferContext, Selection };
use interface::InterfaceContext;
use input::Action;
use system::{ Filebuffer, LanguageManager };

use sfml::system::Vector2f;
use sfml::graphics::*;

pub use self::theme::TextboxTheme;

macro_rules! handle_return_none {
    ($expression: expr) => ({
        $expression;
        return (true, None);
    })
}

pub struct TextBox {
    pub description: SharedString,
    pub textbuffer: Textbuffer,
    pub textbuffer_context: TextbufferContext,
    pub filebuffer: Filebuffer,
    pub selection: Selection,
    pub displacement: usize,
}

impl TextBox {

    pub fn new(language_manager: &mut LanguageManager, description: &'static str, displacement: usize) -> Self {
        let language = SharedString::from("dialogue");
        let textbuffer = Textbuffer::new(0, Vector2f::new(400., 50.), Vector2f::new(0., 0.), ' ', false, false, false);

        Self {
            description: SharedString::from(description),
            textbuffer: textbuffer,
            textbuffer_context: TextbufferContext::textbox(),
            filebuffer: Filebuffer::new(language_manager, language, SharedString::from(" ")),
            selection: Selection::new(0, 0, 0),
            displacement: displacement,
        }
    }

    pub fn get(&self) -> SharedString {
        let mut padded_text = self.filebuffer.get_text();
        padded_text.pop();
        return padded_text;
    }

    pub fn set_text(&mut self, language_manager: &mut LanguageManager, text: SharedString) {
        self.textbuffer.set_text(language_manager, &mut self.filebuffer, format_shared!("{} ", text));
        self.textbuffer.select_last_character(&mut self.filebuffer);
    }

    pub fn set_text_without_save(&mut self, language_manager: &mut LanguageManager, text: SharedString) {
        self.textbuffer.set_text_without_save(language_manager, &mut self.filebuffer, format_shared!("{} ", text));
        self.textbuffer.select_last_character(&mut self.filebuffer);
    }

    pub fn clear(&mut self, language_manager: &mut LanguageManager) {
        self.textbuffer.set_text(language_manager, &mut self.filebuffer, format_shared!(" "));
        self.textbuffer.reset(&mut self.filebuffer);
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.textbuffer.add_character(&self.textbuffer_context, language_manager, &mut self.filebuffer, character);
    }

    pub fn handle_action(&mut self, language_manager: &mut LanguageManager, action: Action) -> Option<Action> {
        return self.textbuffer.handle_action(&self.textbuffer_context, language_manager, &mut self.filebuffer, action);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        let dialogue_height = theme.height * interface_context.font_size as f32;

        // make sure that size.y > dialogue_height ?

        self.textbuffer.resize(interface_context, Vector2f::new(size.x, dialogue_height));
        self.textbuffer.set_position(position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme, focused: bool) {

        let dialogue_height = theme.height * interface_context.font_size as f32;
        let textbuffer_theme = match focused {
            true => &theme.focused_textbox_theme.textbuffer_theme,
            false => &theme.unfocused_textbox_theme.textbuffer_theme,
        };

        self.textbuffer.render(framebuffer, interface_context, &self.textbuffer_context, textbuffer_theme, &self.filebuffer, dialogue_height, focused);
    }
}
