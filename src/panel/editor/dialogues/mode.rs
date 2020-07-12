use kami::VectorString;
use super::super::Selection;

#[derive(Clone)]
pub enum DialogueMode {
    None,
    Error(VectorString),
    OpenFile,
    SetLanguage,
    FindReplace(Vec<Selection>),
}
