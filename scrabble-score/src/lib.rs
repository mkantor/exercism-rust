use std::ascii::AsciiExt;

fn letter_score(letter: char) -> u32 {
    match letter.to_ascii_lowercase() {
        'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
        'd'|'g' => 2,
        'b'|'c'|'m'|'p' => 3,
        'f'|'h'|'v'|'w'|'y' => 4,
        'k' => 5,
        'j'|'x' => 8,
        'q'|'z' => 10,
        _ => 0,
    }
}

pub fn score(word: &str) -> u32 {
    word.chars().fold(0, |word_score, letter| {
        word_score + letter_score(letter)
    })
}
