use sfml::window::Key;
use super::Modifiers;

macro_rules! match_modifier_state {
    ($modifiers: expr, $field: ident, $state:expr) => ({
        if $modifiers.$field != $state {
            return false;
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    trigger: Key,
    included: Modifiers,
    excluded: Modifiers,
}

impl Binding {

    pub fn new(trigger: Key, included: Modifiers, excluded: Modifiers) -> Self {
        Self {
            trigger: trigger,
            included: included,
            excluded: excluded,
        }
    }

    pub fn length(&self) -> usize {
        return self.included.length() + self.excluded.length();
    }

    pub fn matches(&self, trigger: &Key, modifiers: &Modifiers) -> bool {

        if self.trigger != *trigger {
            return false;
        }

        if !self.included.matches_state(&modifiers, true) {
            return false;
        }

        if !self.excluded.matches_state(&modifiers, false) {
            return false;
        }

        return true;
    }
}
