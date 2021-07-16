mod action;
mod history;
mod filebuffer;
mod language;
mod manager;
mod instance;
mod window;

use self::window::PoetWindow;
use self::history::History;

pub use self::action::BufferAction;
pub use self::filebuffer::Filebuffer;
pub use self::instance::Instance;
pub use self::language::LanguageManager;
pub use self::manager::ResourceManager;

pub fn subtract_or_zero(left: usize, right: usize) -> usize {
    match left < right {
        true => return 0,
        false => return left - right,
    }
}
