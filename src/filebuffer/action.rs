use seamonkey::*;

use selection::SelectionMode;

#[derive(Clone, Debug)]
pub enum BufferAction {
    InsertText(usize, SharedString, usize),
    RemoveText(usize, SharedString, usize),
    AddSelection(usize, usize, usize, usize, usize),
    RemoveSelection(usize, usize, usize, usize, usize),
    ChangePrimaryIndex(usize, usize, usize, usize),
    ChangeSecondaryIndex(usize, usize, usize, usize),
    ChangeOffset(usize, usize, usize, usize),
    ChangeSelectionMode(usize, SelectionMode, SelectionMode),
}

impl BufferAction {

    pub fn is_text(&self) -> bool {
        match self {
            BufferAction::InsertText(..) => return true,
            BufferAction::RemoveText(..) => return true,
            _other => return false,
        }
    }

    pub fn is_other_text(&self, current_id: usize) -> bool {
        match self {
            BufferAction::InsertText(window_id, ..) => return current_id != *window_id,
            BufferAction::RemoveText(window_id, ..) => return current_id != *window_id,
            _other => return false,
        }
    }

    pub fn is_selection(&self, current_id: usize) -> bool {
        match self {
            BufferAction::AddSelection(window_id, ..) => return current_id == *window_id,
            BufferAction::RemoveSelection(window_id, ..) => return current_id == *window_id,
            BufferAction::ChangePrimaryIndex(window_id, ..) => return current_id == *window_id,
            BufferAction::ChangeSecondaryIndex(window_id, ..) => return current_id == *window_id,
            BufferAction::ChangeOffset(window_id, ..) => return current_id == *window_id,
            BufferAction::ChangeSelectionMode(window_id, ..) => return current_id == *window_id,
            _other => return false,
        }
    }

    pub fn invert(self) -> Self {
        match self {
            BufferAction::InsertText(window_id, text, index) => return BufferAction::RemoveText(window_id, text, index),
            BufferAction::RemoveText(window_id, text, index) => return BufferAction::InsertText(window_id, text, index),
            BufferAction::AddSelection(window_id, index, primary_index, secondary_index, offset) => return BufferAction::RemoveSelection(window_id, index, primary_index, secondary_index, offset),
            BufferAction::RemoveSelection(window_id, index, primary_index, secondary_index, offset) => return BufferAction::AddSelection(window_id, index, primary_index, secondary_index, offset),
            BufferAction::ChangePrimaryIndex(window_id, index, previous, new) => return BufferAction::ChangePrimaryIndex(window_id, index, new, previous),
            BufferAction::ChangeSecondaryIndex(window_id, index, previous, new) => return BufferAction::ChangeSecondaryIndex(window_id, index, new, previous),
            BufferAction::ChangeOffset(window_id, index, previous, new) => return BufferAction::ChangeOffset(window_id, index, new, previous),
            BufferAction::ChangeSelectionMode(window_id, previous, new) => return BufferAction::ChangeSelectionMode(window_id, new, previous),
        }
    }
}
