use seamonkey::*;

#[derive(Clone, Debug)]
pub enum BufferAction {
    InsertText(SharedString, usize), // text, buffer_index
    RemoveText(SharedString, usize), // text, buffer_index
    //AddSelection(usize), // buffer_index
    //RemoveSelection(usize), // buffer_index
    //MoveSelection(usize, usize, usize), // primary_index, secondary_index, buffer_index
}

impl BufferAction {

    pub fn invert(&self) -> Self {
        match self.clone() {
            BufferAction::InsertText(text, index) => return BufferAction::RemoveText(text, index),
            BufferAction::RemoveText(text, index) => return BufferAction::InsertText(text, index),
        }
    }
}
