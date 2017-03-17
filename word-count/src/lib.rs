use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let normalized_words = phrase.split(|c: char| {
        !c.is_alphanumeric()
    }).filter(|word| {
        !word.is_empty()
    }).map(str::to_lowercase);

    let mut counts = HashMap::new();
    for word in normalized_words {
        let counter = counts.entry(word).or_insert(0);
        *counter += 1;
    }

    counts
}
