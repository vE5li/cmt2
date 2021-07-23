use seamonkey::*;

use std::collections::HashMap;

use filebuffer::Filebuffer;
use selection::Selection;

pub struct FilebufferManager {
    filebuffers: HashMap<String, Filebuffer>,
    buffer_index: usize,
}

impl FilebufferManager {

    pub fn new() -> Self {
        return Self {
            filebuffers: HashMap::new(),
            buffer_index: 0,
        }
    }

    pub fn insert(&mut self, file_name: String, filebuffer: Filebuffer) {
        self.filebuffers.insert(file_name, filebuffer);
    }

    pub fn remove(&mut self, file_name: &str) {
        self.filebuffers.remove(file_name);
    }

    pub fn get(&self, file_name: &str) -> &Filebuffer {
        return self.filebuffers.get(file_name).unwrap();
    }

    pub fn get_mut(&mut self, file_name: &str) -> &mut Filebuffer {
        return self.filebuffers.get_mut(file_name).unwrap();
    }

    pub fn contains(&self, file_name: &str) -> bool {
        return self.filebuffers.get(file_name).is_some();
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Filebuffer> {
        return self.filebuffers.iter();
    }

    pub fn next_index(&mut self) -> usize {
        self.buffer_index += 1;
        return self.buffer_index;
    }
}
