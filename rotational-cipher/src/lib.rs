use std::ascii::AsciiExt;

// This gets mangled by rustfmt 0.8.0.
// See <https://github.com/rust-lang-nursery/rustfmt/issues/1401>.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn rotate(text: &str, key: u8) -> String {
    text.chars()
        .map(|character| {
            if character.is_ascii() && character.is_alphabetic() {
                let offset = if character.is_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };

                let character_num = character as u8 - offset;
                let rotated_character_num = (character_num + key) % 26;
                (rotated_character_num + offset) as char
            } else {
                character
            }
        })
        .collect()
}
