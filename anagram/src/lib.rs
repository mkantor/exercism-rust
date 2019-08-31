use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let normalized_word = word.to_lowercase();
    let word_histogram = Histogram::new(normalized_word.chars());

    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let normalized_possible_anagram = possible_anagram.to_lowercase();
            if normalized_possible_anagram == normalized_word {
                false // Words are not their own anagrams.
            } else {
                let possible_anagram_histogram =
                    Histogram::new(normalized_possible_anagram.chars());
                possible_anagram_histogram == word_histogram
            }
        })
        .cloned()
        .collect()
}

#[derive(PartialEq, Eq)]
struct Histogram<K: Eq + Hash> {
    counts: HashMap<K, usize>,
}

impl<K: Eq + Hash> Histogram<K> {
    fn new<I: IntoIterator<Item = K>>(items: I) -> Self {
        let mut counts = HashMap::new();
        for item in items {
            let counter = counts.entry(item).or_insert(0);
            *counter += 1;
        }
        Histogram { counts }
    }
}
