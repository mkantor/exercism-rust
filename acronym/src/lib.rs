pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut previous_character = None;
    for character in phrase.chars() {
        if is_part_of_acronym(character, previous_character) {
            acronym.push(character);
        }
        previous_character = Some(character);
    }
    acronym.to_uppercase()
}

fn is_part_of_acronym(character: char, previous_character: Option<char>) -> bool {
    character.is_alphabetic() &&
    match previous_character {
        None => true,
        Some(p) => p == '-' || p.is_whitespace() || (p.is_lowercase() && character.is_uppercase()),
    }
}
