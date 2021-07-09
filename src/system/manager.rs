use seamonkey::*;

use std::collections::HashMap;

use elements::Selection;
use system::Filebuffer;

pub struct ResourceManager {
    pub filebuffers: HashMap<String, Filebuffer>, // filename -> text
    pub languages: HashMap<String, seamonkey::tokenize::Tokenizer>, // language name -> tokenizer
    buffer_index: usize,
}

impl ResourceManager {

    pub fn new() -> Self {
        return Self {
            filebuffers: HashMap::new(),
            languages: HashMap::new(),
            buffer_index: 0,
        }
    }

    pub fn next_index(&mut self) -> usize {
        self.buffer_index += 1;
        return self.buffer_index;
    }
}
