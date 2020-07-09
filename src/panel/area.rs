#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub offset: usize,
    pub width: usize,
    pub height: usize,
}

impl Area {

    pub fn new(width: usize, height: usize, offset: usize) -> Self {
        Self {
            offset: offset,
            width: width,
            height: height,
        }
    }
}
