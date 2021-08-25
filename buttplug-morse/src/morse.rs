use crate::{
    letters::{ERROR, LETTERS},
    options::MorseOptions,
};

/// A single morse signal; A dot or a dash.
#[derive(Debug, Clone, Copy)]
pub enum Signal {
    Dot,
    Dash,
}

impl Signal {
    /// Returns the duration of this signal.
    pub fn duration(&self, options: &MorseOptions) -> u64 {
        match self {
            Signal::Dot => options.dot_length,
            Signal::Dash => options.dash_length,
        }
    }
}

/// A letter of morse code.
#[derive(Debug, Clone, Copy)]
pub struct Letter {
    pub(crate) signals: &'static [Signal],
}

impl Letter {
    /// Tries to create a new [Letter]. If the character provided cannot be represented as a single morse letter,
    /// this function returns [None].
    pub fn new(character: char) -> Option<Letter> {
        LETTERS.get(&character).map(|signals| Letter { signals })
    }

    /// Get a reference to the letter's signals.
    pub fn signals(&self) -> &'static [Signal] {
        self.signals
    }
}

/// A word of morse code, not including whitespace.
pub struct Word {
    letters: Vec<Letter>,
}

impl Word {
    /// Create a new [Word] from a string slice. This function will return [None] if the string slice contains any whitespace
    /// (or otherwise unrepresentable) characters.
    pub fn new(word: &str) -> Option<Self> {
        let result = word
            .chars()
            .map(|character| Letter::new(character.to_ascii_lowercase()))
            .collect::<Option<Vec<_>>>();

        result.map(|letters| Word { letters })
    }

    /// Create a new [Word] from a string slice. If any unrepresentable characters are encountered, they will be
    /// transformed into the more error character.
    pub fn new_lossy(word: &str) -> Self {
        let letters = word
            .chars()
            .map(|character| Letter::new(character.to_ascii_lowercase()).unwrap_or(ERROR))
            .collect::<Vec<_>>();

        Word { letters }
    }

    /// Get a reference to the word's letters.
    pub fn letters(&self) -> &[Letter] {
        self.letters.as_slice()
    }
}

/// A full morse sentence.
pub struct Sentence {
    words: Vec<Word>,
}

impl Sentence {
    /// Create a new morse sentence, returning [None] if an unrepresentable character is encountered, or the result
    /// wrapped in [Some] otherwise.
    pub fn new(sentence: &str) -> Option<Self> {
        let result = sentence
            .split_whitespace()
            .map(Word::new)
            .collect::<Option<Vec<_>>>();

        result.map(|words| Sentence { words })
    }

    /// Create a new morse sentence, replacing any unrepresentable characters with the morse error character,
    /// represented as 8 dots.
    pub fn new_lossy(sentence: &str) -> Self {
        let words = sentence
            .split_whitespace()
            .map(Word::new_lossy)
            .collect::<Vec<_>>();

        Sentence { words }
    }

    /// Get a reference to the sentence's words.
    pub fn words(&self) -> &[Word] {
        self.words.as_slice()
    }
}
