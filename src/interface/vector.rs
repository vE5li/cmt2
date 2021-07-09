use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Vector4f {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Vector4f {

    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        return Self {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    pub fn with(value: f32) -> Self {
        return Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}


impl Mul for Vector4f {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        return Self {
            left: self.left * right.left,
            top: self.top * right.top,
            right: self.right * right.right,
            bottom: self.bottom * right.bottom,
        }
    }
}
