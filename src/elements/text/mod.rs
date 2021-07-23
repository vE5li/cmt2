use seamonkey::*;

use sfml::graphics::{ RenderTexture, Transformable, RenderTarget };
use sfml::system::Vector2f;

use themes::TextTheme;
use interface::InterfaceContext;

pub struct Text { }

impl Text {

    pub fn render(framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &TextTheme, text: &SharedString, size: Vector2f, position: Vector2f) {

        let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;

        let mut character = sfml::graphics::Text::default();
        character.set_font(&interface_context.font);
        character.set_character_size(interface_context.font_size as u32);
        character.set_outline_thickness(interface_context.font_size as f32 * theme.border_width);
        character.set_outline_color(theme.border_color);
        character.set_fill_color(theme.text_color);
        character.set_style(theme.text_style);
        character.set_position(position);

        for index in 0..text.len() {
            character.set_string(&format!("{}", text[index]));
            framebuffer.draw(&character);
            character.move_(Vector2f::new(character_scaling, 0.0));
        }
    }
}
