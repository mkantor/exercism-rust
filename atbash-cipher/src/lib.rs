use std::ascii::AsciiExt;

pub fn encode(plaintext: &str) -> String {
    plaintext.chars()
        .filter_map(normalize_input)
        .map(atbash)
        .enumerate()
        .fold(String::new(), |mut ciphertext, (i, character)| {
            // The requirements ask for encoded strings to have a space every five chars.
            if i % 5 == 0 && i != 0 {
                ciphertext.push(' ');
            }
            ciphertext.push(character);
            ciphertext
        })
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext.chars()
        .filter_map(normalize_input)
        .map(atbash)
        .collect()
}

fn normalize_input(character: char) -> Option<char> {
    if character.is_alphanumeric() && character.is_ascii() {
        Some(character.to_ascii_lowercase())
    } else {
        None
    }
}

fn atbash(character: char) -> char {
    if character.is_alphabetic() {
        // Here's the interesting part of the cipher. This will "flip" the alphabet: 'a' => 'z',
        // 'b' => 'y', 'm' => 'n', 'z' => 'a', etc.
        ('z' as u8 + 'a' as u8 - character as u8) as char
    } else {
        character
    }
}
