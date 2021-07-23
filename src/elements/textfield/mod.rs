use seamonkey::*;

use sfml::system::Vector2f;
use sfml::graphics::RenderTexture;

use themes::{ TextfieldTheme, Alignment };
use elements::{ Text, Field };
use interface::InterfaceContext;

pub struct Textfield { }

impl Textfield {

    pub fn render(framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &TextfieldTheme, text: &SharedString, size: Vector2f, position: Vector2f, scaler: f32) {

        let text_position = match &theme.alignment {

            Alignment::Left => {
                let left_position = theme.text_offset * interface_context.font_size as f32;
                position + Vector2f::new(left_position, 0.0)
            }

            Alignment::Center => {
                let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
                let text_size = text.len() as f32 * character_scaling;
                let left_position = (size.x - text_size) / 2.0;
                position + Vector2f::new(left_position, 0.0)
            }

            Alignment::Right => {
                let character_scaling = interface_context.character_spacing * interface_context.font_size as f32;
                let text_size = text.len() as f32 * character_scaling;
                let left_position = size.x - text_size - theme.text_offset * interface_context.font_size as f32;
                position + Vector2f::new(left_position as f32, 0.0)
            }
        };

        Field::render(framebuffer, interface_context, &theme.field_theme, size, position, scaler);
        Text::render(framebuffer, interface_context, &theme.text_theme, text, size, text_position);
    }
}
