use seamonkey::*;
use seamonkey::tokenize::Tokenizer;

use std::collections::HashMap;

use elements::Selection;
use system::Filebuffer;

pub struct LanguageManager {
    pub tokenizers: HashMap<String, Tokenizer>,
}

impl LanguageManager {

    pub fn new() -> Self {
        return Self {
            tokenizers: HashMap::new(),
        }
    }

    fn load_language(language: &SharedString) -> Status<Tokenizer> {
        let file_path = format_shared!("/home/.config/poet/languages/{}.data", language);
        let tokenizer_map = confirm!(read_map(&file_path)); // confirm!(read_map(&file_path), Message, "...");
        return Tokenizer::new(&tokenizer_map);
    }

    pub fn get_load(&mut self, language: &SharedString) -> Status<&Tokenizer> {
        let language_string = language.serialize();

        if self.tokenizers.get(&language_string).is_none() {
            let file_path = format_shared!("/home/.config/poet/languages/{}.data", language);
            let tokenizer_map = confirm!(read_map(&file_path)); // confirm!(read_map(&file_path), Message, "...");
            let tokenizer = confirm!(Tokenizer::new(&tokenizer_map));
            self.tokenizers.insert(language_string.clone(), tokenizer);
        }

        return success!(self.tokenizers.get(&language_string).unwrap());
    }
}
