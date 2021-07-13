use seamonkey::SharedString;
use elements::Selection;

#[derive(Clone)]
pub enum DialogueMode {
    None,
    OpenFile,
    LoadedBuffers,
    SetLanguage,
    SetTheme,
    FindReplace(Vec<Selection>),
    Action,
}
