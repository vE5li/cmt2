mod shape;
mod theme;

use sfml::system::Vector2f;
use sfml::graphics::{ RenderTexture, CustomShape, Transformable, RenderTarget, Shape };

use self::shape::RoundedRectangle;

use interface::{ InterfaceContext, Vector4f };

pub use self::theme::FieldTheme;

pub struct Field { }

impl Field {

    pub fn render(framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &FieldTheme, size: Vector2f, position: Vector2f, scaler: f32) {
        let rounded = RoundedRectangle::new(size, theme.corner_radius * Vector4f::with(scaler));
        let mut shape = CustomShape::new(Box::new(rounded));
        shape.set_fill_color(theme.background_color);
        shape.set_outline_thickness(interface_context.font_size as f32 * theme.border_width);
        shape.set_outline_color(theme.border_color);
        shape.set_position(position);
        framebuffer.draw(&shape);
    }
}
