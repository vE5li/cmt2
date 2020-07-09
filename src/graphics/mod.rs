mod rounded;

pub use self::rounded::RoundedRectangle;

use kami::VectorString;
use sfml::system::Vector2f;
use sfml::graphics::*;

pub fn draw_spaced_text(framebuffer: &mut RenderTexture, text: &mut Text, mut position: Vector2f, string: &VectorString, character_scaling: f32) {
    for index in 0..string.len() {
        text.set_position(position);
        text.set_string(&format!("{}", string[index]));
        framebuffer.draw(text);
        position.x += character_scaling;
    }
}
