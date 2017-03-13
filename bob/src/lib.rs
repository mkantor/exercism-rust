pub fn reply(input: &str) -> &str {
    if input.is_empty() {
        "Fine. Be that way!"
    } else if input.ends_with("?") {
        "Sure."
    } else if !input.chars().any(|char| char.is_lowercase()) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
