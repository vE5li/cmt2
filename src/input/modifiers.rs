#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub system: bool,
}

impl Modifiers {

    pub fn new() -> Self {
        Self {
            shift: false,
            control: false,
            alt: false,
            system: false,
        }
    }

    pub fn from(shift: bool, control: bool, alt: bool, system: bool) -> Self {
        Self {
            shift: shift,
            control: control,
            alt: alt,
            system: system,
        }
    }

    pub fn length(&self) -> usize {
        let mut count = 0;

        if self.shift {
            count += 1;
        }

        if self.control {
            count += 1;
        }

        if self.alt {
            count += 1;
        }

        if self.system {
            count += 1;
        }

        return count;
    }

    pub fn matches_state(&self, modifiers: &Modifiers, state: bool) -> bool {

        if self.shift && modifiers.shift != state {
            return false;
        }

        if self.control && modifiers.control != state {
            return false;
        }

        if self.alt && modifiers.alt != state {
            return false;
        }

        if self.system && modifiers.system != state {
            return false;
        }

        return true;
    }
}
