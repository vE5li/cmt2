#[derive(Copy, Clone, Debug)]
pub struct LineInfo {
    pub number: usize,
    pub index: usize,
    pub length: usize,
    pub highlighted: bool
}

impl LineInfo {

    pub fn new(number: usize, index: usize, length: usize) -> Self {
        return Self {
            number: number,
            index: index,
            length: length,
            highlighted: false,
        }
    }
}
