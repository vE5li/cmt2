use seamonkey::*;

use std::time::{ Duration, SystemTime };

use elements::Selection;
use system::BufferAction;

const COMBINE_DURATION: f32 = 0.5;

#[derive(Clone)]
pub struct History {
    actions: Vec<Vec<BufferAction>>,
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

    pub fn insert_text(&mut self, index: usize, text: SharedString) -> bool {
        let action = BufferAction::InsertText(text, index);
        let combined = self.update_timestamp();

        match !self.actions.is_empty() && combined {
            true => self.actions.last_mut().unwrap().push(action),
            false => self.actions.push(vec![action]),
        }

        return !combined;
    }

    pub fn remove_text(&mut self, text: SharedString, index: usize) -> bool {
        let action = BufferAction::RemoveText(text, index);
        let combined = self.update_timestamp();

        match combined {
            true => self.actions.last_mut().unwrap().push(action),
            false => self.actions.push(vec![action]),
        }

        return !combined;
    }

    pub fn pop_until(&mut self, index: usize) {
        for _ in index..self.actions.len() {
            self.actions.pop().unwrap();
        }
    }

    pub fn get(&self, index: usize) -> Vec<BufferAction> {
        return self.actions[index].clone();
    }

    pub fn length(&mut self) -> usize {
        return self.actions.len();
    }
}
