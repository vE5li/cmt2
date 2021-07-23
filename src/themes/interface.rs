use seamonkey::*;

use themes::*;

pub struct InterfaceTheme {
    pub name: SharedString,
    pub textbuffer_theme: TextbufferTheme,
    pub dialogue_theme: DialogueTheme,
    pub message_theme: MessageTheme,
}

impl InterfaceTheme {

    pub fn load(theme: Option<Data>, name: &SharedString) -> Self {
        return Self {
            name: name.clone(),
            textbuffer_theme: TextbufferTheme::load(get_subtheme(&theme, "textbuffer")),
            dialogue_theme: DialogueTheme::load(get_subtheme(&theme, "dialogue")),
            message_theme: MessageTheme::load(get_subtheme(&theme, "message")),
        }
    }
}
