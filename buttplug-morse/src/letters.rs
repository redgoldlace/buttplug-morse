use phf::{phf_map, Map};

use crate::morse::{Letter, Signal};

pub static LETTERS: Map<char, &'static [Signal]> = {
    use Signal::*;

    phf_map! {
        'a' => &[Dot, Dash],
        'b' => &[Dash, Dot, Dot, Dot],
        'c' => &[Dash, Dot, Dash, Dot],
        'd' => &[Dash, Dot, Dot],
        'e' => &[Dot],
        'f' => &[Dot, Dot, Dash, Dot],
        'g' => &[Dash, Dash, Dot],
        'h' => &[Dot, Dot, Dot, Dot],
        'i' => &[Dot, Dot],
        'j' => &[Dot, Dash, Dash, Dash],
        'k' => &[Dash, Dot, Dash],
        'l' => &[Dot, Dash, Dot, Dot],
        'm' => &[Dash, Dash],
        'n' => &[Dash, Dot],
        'o' => &[Dash, Dash, Dash],
        'p' => &[Dot, Dash, Dash, Dot],
        'q' => &[Dash, Dash, Dot, Dash],
        'r' => &[Dot, Dash, Dot],
        's' => &[Dot, Dot, Dot],
        't' => &[Dash],
        'u' => &[Dot, Dot, Dash],
        'v' => &[Dot, Dot, Dot, Dash],
        'w' => &[Dot, Dash, Dash],
        'x' => &[Dash, Dot, Dot, Dash],
        'y' => &[Dash, Dot, Dash, Dash],
        'z' => &[Dash, Dash, Dot, Dot],
        '0' => &[Dash, Dash, Dash, Dash, Dash],
        '1' => &[Dot, Dash, Dash, Dash, Dash],
        '2' => &[Dot, Dot, Dash, Dash, Dash],
        '3' => &[Dot, Dot, Dot, Dash, Dash],
        '4' => &[Dot, Dot, Dot, Dot, Dash],
        '5' => &[Dot, Dot, Dot, Dot, Dot],
        '6' => &[Dash, Dot, Dot, Dot, Dot],
        '7' => &[Dash, Dash, Dot, Dot, Dot],
        '8' => &[Dash, Dash, Dash, Dot, Dot],
        '9' => &[Dash, Dash, Dash, Dash, Dot],
        '.' => &[Dot, Dash, Dot, Dash, Dot, Dash],
        ',' => &[Dash, Dash, Dot, Dot, Dash, Dash],
        '?' => &[Dot, Dot, Dash, Dash, Dot, Dot,],
        '\'' => &[Dot, Dash, Dash, Dash, Dash, Dot],
        '!' => &[Dot, Dash, Dot, Dash, Dash, Dash],
        '/' => &[Dash, Dot, Dot, Dash, Dot],
        '(' => &[Dash, Dot, Dash, Dash, Dot],
        ')' => &[Dash, Dot, Dash, Dash, Dot, Dash],
        '&' => &[Dot, Dash, Dash, Dash, Dash],
        ':' => &[Dash, Dash, Dash, Dot, Dot, Dot],
        ';' => &[Dash, Dot, Dash, Dot, Dash, Dot],
        '=' => &[Dash, Dot, Dot, Dot, Dash],
        '+' => &[Dot, Dash, Dot, Dash, Dot],
        '-' => &[Dash, Dot, Dot, Dot, Dot, Dash],
        '_' => &[Dot, Dot, Dash, Dash, Dot, Dash],
        '"' => &[Dot, Dash, Dot, Dot, Dash, Dot],
        '$' => &[Dot, Dot, Dot, Dash, Dot, Dot, Dash],
        '@' => &[Dot, Dash, Dash, Dot, Dash, Dot],
    }
};

pub static ERROR: Letter = Letter {
    signals: &[Signal::Dot; 8],
};
