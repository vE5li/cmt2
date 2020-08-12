use sfml::graphics::*;
use sfml::system::Vector2f;

#[derive(Clone)]
pub struct RoundedRectangle {
    points: Vec<Vector2f>,
}

impl RoundedRectangle {

    pub fn new(width: f32, height: f32, top_left_radius: f32, top_right_radius: f32, bottom_right_radius: f32, bottom_left_radius: f32) -> Self {
        let quarter_rotation = std::f32::consts::PI / 2.0;
        let mut points = Vec::new();

        if top_left_radius < 1.0 {
            points.push(Vector2f::new(0.0, 0.0));
        } else {
            let point_count = 3 + (top_left_radius * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(0.0, top_left_radius));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(top_left_radius, 0.0));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = top_left_radius - top_left_radius * angle.cos();
                    let y = top_left_radius - top_left_radius * angle.sin();
                    points.push(Vector2f::new(x, y));
                }
            }
        }

        if top_right_radius < 1.0 {
            points.push(Vector2f::new(width, 0.0));
        } else {
            let point_count = 3 + (top_right_radius * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(width - top_right_radius, 0.0));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(width, top_right_radius));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = top_right_radius - top_right_radius * angle.cos();
                    let y = top_right_radius - top_right_radius * angle.sin();
                    points.push(Vector2f::new(width - x, y));
                }
            }
        }

        if bottom_right_radius < 1.0 {
            points.push(Vector2f::new(width, height));
        } else {
            let point_count = 3 + (bottom_right_radius * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(width, height - bottom_right_radius));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(width - bottom_right_radius, height));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = bottom_right_radius - bottom_right_radius * angle.cos();
                    let y = bottom_right_radius - bottom_right_radius * angle.sin();
                    points.push(Vector2f::new(width - x, height - y));
                }
            }
        }

        if bottom_left_radius < 1.0 {
            points.push(Vector2f::new(0.0, height));
        } else {
            let point_count = 3 + (bottom_left_radius * 0.5) as u32;
            for index in 0..point_count {
                if index == 0 {
                    points.push(Vector2f::new(bottom_left_radius, height));
                } else if index == point_count - 1 {
                    points.push(Vector2f::new(0.0, height - bottom_left_radius));
                } else {
                    let angle = quarter_rotation / (point_count - 1) as f32 * index as f32;
                    let x = bottom_left_radius - bottom_left_radius * angle.cos();
                    let y = bottom_left_radius - bottom_left_radius * angle.sin();
                    points.push(Vector2f::new(x, height - y));
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
