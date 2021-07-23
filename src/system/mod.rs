mod instance;
mod window;

use self::window::PoetWindow;

pub use self::instance::Instance;

pub fn subtract_or_zero(left: usize, right: usize) -> usize {
    match left < right {
        true => return 0,
        false => return left - right,
    }
}
