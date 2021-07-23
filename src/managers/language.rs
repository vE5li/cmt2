use seamonkey::*;
use seamonkey::tokenize::Tokenizer;

#[cfg(feature = "debug")]
use debug::*;

use std::collections::HashMap;

use selection::Selection;
use filebuffer::Filebuffer;

pub struct LanguageManager {
    pub tokenizers: HashMap<String, Tokenizer>,
}

impl LanguageManager {

    pub fn new() -> Self {
        return Self {
            tokenizers: HashMap::new(),
        }
    }

    pub fn get_load(&mut self, language: &SharedString) -> Status<&Tokenizer> {
        let language_string = language.serialize();

        if self.tokenizers.get(&language_string).is_none() {

            #[cfg(feature = "debug")]
            let timer = Timer::new_dynamic(format!("load language {}", language_string));

            let file_path = format_shared!("/home/.config/poet/languages/{}.data", language);
            let tokenizer_map = confirm!(read_map(&file_path)); // confirm!(read_map(&file_path), Message, "...");
            let tokenizer = confirm!(Tokenizer::new(&tokenizer_map));
            self.tokenizers.insert(language_string.clone(), tokenizer);

            #[cfg(feature = "debug")]
            timer.stop();
        }

        return success!(self.tokenizers.get(&language_string).unwrap());
    }
}
