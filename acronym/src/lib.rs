pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .flat_map(|word| word.split('-'))
        .map(|word| {
            let is_all_caps = word.chars()
                .filter(|&character| character.is_alphabetic())
                .all(char::is_uppercase);
            if is_all_caps {
                word[0..1].to_string()
            } else {
                word.chars().enumerate()
                    .filter(|&(i, character)| {
                        i == 0 || character.is_uppercase()
                    })
                    .flat_map(|(_, character)| {
                        character.to_uppercase()
                    })
                    .collect()
            }
        })
        .collect()
}
