use sfml::graphics::*;
use sfml::system::Vector2f;

#[derive(Clone)]
pub struct RoundedRectangle {
    points: Vec<Vector2f>,
}

impl RoundedRectangle {

    pub fn new(width: f32, height: f32, top_left_radius: f32, top_right_radius: f32, bottom_right_radius: f32, bottom_left_radius: f32) -> Self {

        let mut x = 0.0;
        let mut y = 0.0;
        let mut points = Vec::new();

        let top_right_points = 3 + (top_right_radius * 0.5) as usize;
        for index in 0..top_right_points {
            //x += top_right_radius / top_right_points as f32;

            x = match index == top_right_points - 1 {
                true => top_right_radius,
                false => x + top_right_radius / top_right_points as f32,
            };

            y = (top_right_radius * top_right_radius - x * x).sqrt();
            points.push(Vector2f::new(x + width - top_right_radius, top_right_radius - y));
        }

        y = 0.0;
        let bottom_right_points = 3 + (bottom_right_radius * 0.5) as usize;
        for index in 0..bottom_right_points {

            y = match index == bottom_right_points - 1 {
                true => bottom_right_radius,
                false => y + bottom_right_radius / bottom_right_points as f32,
            };

            //y += bottom_right_radius / bottom_right_points as f32;
            x = (bottom_right_radius * bottom_right_radius - y * y).sqrt();
            points.push(Vector2f::new(width + x - bottom_right_radius, height - bottom_right_radius + y));
        }

        x = 0.0;
        let bottom_left_points = 3 + (bottom_left_radius * 0.5) as usize;
        for index in 0..bottom_left_points {

            x = match index == bottom_left_points - 1 {
                true => bottom_left_radius,
                false => x + bottom_left_radius / bottom_left_points as f32,
            };

            //x += bottom_left_radius / bottom_left_points as f32;
            y = (bottom_left_radius * bottom_left_radius - x * x).sqrt();
            points.push(Vector2f::new(bottom_left_radius - x, height - bottom_left_radius + y));
        }

        y = 0.0;
        let top_left_points = 3 + (top_left_radius * 0.5) as usize;
        for index in 0..top_left_points {

            y = match index == top_left_points - 1 {
                true => top_left_radius,
                false => y + top_left_radius / top_left_points as f32,
            };

            //y += top_left_radius / top_left_points as f32;
            x = (top_left_radius * top_left_radius - y * y).sqrt();
            points.push(Vector2f::new(top_left_radius - x, top_left_radius - y));
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
