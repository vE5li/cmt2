use seamonkey::SharedString;
use super::super::Selection;

#[derive(Clone)]
pub enum DialogueMode {
    None,
    Error(SharedString),
    OpenFile,
    SetLanguage,
    FindReplace(Vec<Selection>),
    Action,
}
