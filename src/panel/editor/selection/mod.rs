mod mode;

pub use self::mode::SelectionMode;

#[derive(Clone, Debug)]
pub struct Selection {
    pub index: usize,
    pub length: usize,
    pub offset: usize,
    pub reversed: bool,
}

impl Selection {

    pub fn new(index: usize, length: usize, offset: usize) -> Self {
        Self {
            index: index,
            length: length,
            offset: offset,
            reversed: false,
        }
    }

    pub fn set_index_offset(&mut self, index: usize, offset: usize) {
        self.index = index;
        self.offset = offset;
    }

    pub fn set_index_length(&mut self, index: usize, length: usize) {
        self.index = index;
        self.length = length;
    }

    pub fn reset(&mut self) {
        self.length = 1;
        self.reversed = false;
    }
}
