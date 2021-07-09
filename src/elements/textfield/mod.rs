mod theme;

use seamonkey::*;

use sfml::system::Vector2f;
use sfml::graphics::RenderTexture;

use elements::{ Text, Field };
use interface::InterfaceContext;

pub use self::theme::TextfieldTheme;

pub struct Textfield { }

impl Textfield {

    pub fn render(framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &TextfieldTheme, text: &SharedString, size: Vector2f, position: Vector2f, scaler: f32) {
        // process alignment
        let text_position = position + Vector2f::new(theme.text_offset * interface_context.font_size as f32, 0.0);

        Field::render(framebuffer, interface_context, &theme.field_theme, size, position, scaler);
        Text::render(framebuffer, interface_context, &theme.text_theme, text, size, text_position);
    }
}
