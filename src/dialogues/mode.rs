use seamonkey::SharedString;
use selection::Selection;

#[derive(Clone)]
pub enum DialogueMode {
    None,
    Open,
    Filebuffers,
    Notes,
    Language,
    Theme,
    Replace(Vec<Selection>),
    Action,
}
