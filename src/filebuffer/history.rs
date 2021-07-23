use seamonkey::*;

use std::time::{ Duration, SystemTime };

use selection::{ Selection, SelectionMode };
use super::{ BufferAction, BufferActionStep };

const COMBINE_DURATION: f32 = 0.5;

#[derive(Clone)]
pub struct History {
    actions: Vec<BufferActionStep>,
    timestamp: SystemTime,
}

impl History {

    pub fn new() -> Self {
        return Self {
            actions: Vec::new(),
            timestamp: SystemTime::now(),
        }
    }

    fn update_timestamp(&mut self) -> bool {
        let elapsed_time = self.timestamp.elapsed().unwrap().as_secs_f32();
        self.timestamp = SystemTime::now();
        return elapsed_time <= COMBINE_DURATION;
    }

    fn append_action(&mut self, action: BufferAction, combine: bool) {
        let combined = combine && self.update_timestamp();
        self.actions.push(BufferActionStep::new(action, combined));
    }

    pub fn insert_text(&mut self, window_id: usize, index: usize, text: SharedString, combine: bool) {
        self.append_action(BufferAction::InsertText(window_id, text, index), combine);
    }

    pub fn remove_text(&mut self, window_id: usize, text: SharedString, index: usize, combine: bool) {
        self.append_action(BufferAction::RemoveText(window_id, text, index), combine);
    }

    pub fn add_selection(&mut self, window_id: usize, index: usize, primary_index: usize, secondary_index: usize, offset: usize, combine: bool) {
        self.append_action(BufferAction::AddSelection(window_id, index, primary_index, secondary_index, offset), combine);
    }

    pub fn remove_selection(&mut self, window_id: usize, index: usize, primary_index: usize, secondary_index: usize, offset: usize, combine: bool) {
        self.append_action(BufferAction::RemoveSelection(window_id, index, primary_index, secondary_index, offset), combine);
    }

    pub fn change_primary_index(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) {
        self.append_action(BufferAction::ChangePrimaryIndex(window_id, index, previous, new), combine);
    }

    pub fn change_secondary_index(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) {
        self.append_action(BufferAction::ChangeSecondaryIndex(window_id, index, previous, new), combine);
    }

    pub fn change_offset(&mut self, window_id: usize, index: usize, previous: usize, new: usize, combine: bool) {
        self.append_action(BufferAction::ChangeOffset(window_id, index, previous, new), combine);
    }

    pub fn change_selection_mode(&mut self, window_id: usize, previous: SelectionMode, new: SelectionMode, combine: bool) {
        self.append_action(BufferAction::ChangeSelectionMode(window_id, previous, new), combine);
    }

    pub fn pop_until(&mut self, index: usize) {
        for _ in index..self.actions.len() {
            self.actions.pop().unwrap();
        }
    }

    pub fn get(&self, index: usize) -> BufferAction {
        return self.actions[index].action.clone();
    }

    pub fn is_action_combined(&self, index: usize) -> bool {
        return self.actions[index].combined;
    }

    pub fn length(&mut self) -> usize {
        return self.actions.len();
    }
}
