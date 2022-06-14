use std::collections::BTreeSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let letters: BTreeSet<char> = sentence
        .chars()
        .map(|letter| match letter.is_ascii_uppercase() {
            true => letter.to_ascii_lowercase(),
            false => letter,
        })
        .filter(|letter| letter.is_ascii_alphabetic())
        .collect();

    letters.len() == 26
}
