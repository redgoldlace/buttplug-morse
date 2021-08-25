#[derive(Debug, Clone, Copy)]
pub struct Intensity(f64);

impl std::ops::Deref for Intensity {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Intensity {
    pub fn new(value: f64) -> Option<Self> {
        (0f64..=1f64).contains(&value).then(|| Intensity(value))
    }
}

#[derive(Debug, Clone)]
pub struct MorseOptions {
    /// The time a dot lasts, in milliseconds.
    pub dot_length: u64,
    /// The time a dash lasts, in milliseconds.
    pub dash_length: u64,
    /// The time a pause between a dot/dash lasts, in milliseconds.
    pub pause_length: u64,
    /// The time a pause between letters lasts, in milliseconds.
    pub letter_pause_length: u64,
    /// The time a space character lasts, in milliseconds.
    pub space_length: u64,
    /// The intensity of the vibration.
    pub intensity: Intensity,
}

impl Default for MorseOptions {
    fn default() -> Self {
        Self {
            dot_length: 100,
            dash_length: 300,
            pause_length: 100,
            letter_pause_length: 300,
            space_length: 700,
            intensity: Intensity(0.5),
        }
    }
}
