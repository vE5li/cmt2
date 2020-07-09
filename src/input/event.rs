use kami::*;
use sfml::window::Key;
use super::Modifiers;

#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
    pub trigger: Key,
    pub modifiers: Modifiers,
}

impl KeyEvent {

    pub fn new(trigger: Key, modifiers: Modifiers) -> Self {
        Self {
            trigger: trigger,
            modifiers: modifiers,
        }
    }
}
