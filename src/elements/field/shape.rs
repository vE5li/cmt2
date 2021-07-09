use sfml::graphics::*;
use sfml::system::Vector2f;

use interface::Vector4f;

#[derive(Clone)]
pub struct RoundedRectangle {
    points: Vec<Vector2f>,
}

impl RoundedRectangle {

    pub fn new(size: Vector2f, corner_radius: Vector4f) -> Self {
        let quarter_rotation = std::f32::consts::PI / 2.0;
        let mut points = Vec::new();

        if corner_radius.left < 1.0 {
            points.push(Vector2f::new(0.0, 0.0));
        } else {
            let point_count = 3 + (corner_radius.left * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(0.0, corner_radius.left));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(corner_radius.left, 0.0));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = corner_radius.left - corner_radius.left * angle.cos();
                    let y = corner_radius.left - corner_radius.left * angle.sin();
                    points.push(Vector2f::new(x, y));
                }
            }
        }

        if corner_radius.top < 1.0 {
            points.push(Vector2f::new(size.x, 0.0));
        } else {
            let point_count = 3 + (corner_radius.top * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(size.x - corner_radius.top, 0.0));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(size.x, corner_radius.top));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = corner_radius.top - corner_radius.top * angle.sin();
                    let y = corner_radius.top - corner_radius.top * angle.cos();
                    points.push(Vector2f::new(size.x - x, y));
                }
            }
        }

        if corner_radius.right < 1.0 {
            points.push(Vector2f::new(size.x, size.y));
        } else {
            let point_count = 3 + (corner_radius.right * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(size.x, size.y - corner_radius.right));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(size.x - corner_radius.right, size.y));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = corner_radius.right - corner_radius.right * angle.cos();
                    let y = corner_radius.right - corner_radius.right * angle.sin();
                    points.push(Vector2f::new(size.x - x, size.y - y));
                }
            }
        }

        if corner_radius.bottom < 1.0 {
            points.push(Vector2f::new(0.0, size.y));
        } else {
            let point_count = 3 + (corner_radius.bottom * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(corner_radius.bottom, size.y));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(0.0, size.y - corner_radius.bottom));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = corner_radius.bottom - corner_radius.bottom * angle.sin();
                    let y = corner_radius.bottom - corner_radius.bottom * angle.cos();
                    points.push(Vector2f::new(x, size.y - y));
                }
            }
        }

        Self {
            points: points,
        }
    }
}

impl CustomShapePoints for RoundedRectangle {

    fn point_count(&self) -> u32 {
        return self.points.len() as u32;
    }

    fn point(&self, point: u32) -> Vector2f {
        return self.points[point as usize];
    }
}
