use super::BufferAction;

#[derive(Clone)]
pub struct BufferActionStep {
    pub action: BufferAction,
    pub combined: bool,
}

impl BufferActionStep {

    pub fn new(action: BufferAction, combined: bool) -> Self {
        return Self {
            action: action,
            combined: combined,
        }
    }
}
