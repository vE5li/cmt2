mod theme;

use seamonkey::*;

use sfml::system::Vector2f;
use sfml::graphics::RenderTexture;

use elements::{ Textfield, TextfieldTheme };
use interface::InterfaceContext;

pub use self::theme::{ PopupTheme, MessageTheme };

pub struct Popup {
    pub size: Vector2f,
    pub position: Vector2f,
}

impl Popup {

    pub fn new() -> Self {
        return Self {
            size: Vector2f::new(0.0, 0.0),
            position: Vector2f::new(0.0, 0.0),
        }
    }

    pub fn update_layout(&mut self, size: Vector2f, position: Vector2f) {
        self.size = size;
        self.position = position;
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &PopupTheme, message: &SharedString) {
        let dialogue_height = theme.height * interface_context.font_size as f32;
        let size = Vector2f::new(self.size.x, dialogue_height);

        Textfield::render(framebuffer, interface_context, &theme.textfield_theme, message, size, self.position, interface_context.font_size as f32);
    }
}
