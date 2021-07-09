use seamonkey::SharedString;
use elements::Selection;

#[derive(Clone)]
pub enum DialogueMode {
    None,
    OpenFile,
    LoadedBuffers,
    SetLanguage,
    FindReplace(Vec<Selection>),
    Action,
}
