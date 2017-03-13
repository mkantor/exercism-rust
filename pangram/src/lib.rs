use std::ascii::AsciiExt;
use std::collections::HashSet;

const ALPHABET_LEN: usize = 26;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.chars().filter(|char| {
        char.is_ascii() && char.is_alphabetic()
    }).map(|char| {
        char.to_ascii_lowercase()
    }).collect::<HashSet<char>>().len() == ALPHABET_LEN
}
