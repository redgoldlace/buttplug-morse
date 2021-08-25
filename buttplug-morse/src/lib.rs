mod letters;
pub mod morse;
pub mod options;
mod play;

pub mod prelude {
    pub use super::options::{Intensity, MorseOptions};
    pub use super::play::{play, Play};
}

pub use letters::{ERROR, LETTERS};
