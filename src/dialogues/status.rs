pub struct DialogueStatus {
    pub handled: bool,
    pub closed: bool,
    pub completed: bool,
}

impl DialogueStatus {

    pub fn handled() -> Self {
        return Self {
            handled: true,
            closed: false,
            completed: false,
        }
    }

    pub fn unhandled() -> Self {
        return Self {
            handled: false,
            closed: false,
            completed: false,
        }
    }

    pub fn aborted() -> Self {
        return Self {
            handled: true,
            closed: true,
            completed: false,
        }
    }

    pub fn completed() -> Self {
        return Self {
            handled: true,
            closed: true,
            completed: true,
        }
    }
}
