use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counter = HashMap::new();

    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut new_s = s.to_ascii_lowercase();
            if let Some(ss) = new_s.strip_prefix('\'') {
                new_s = ss.to_string();
            }
            if let Some(ss) = new_s.strip_suffix('\'') {
                new_s = ss.to_string();
            }
            new_s
        })
        .for_each(|part| {
            if !part.is_empty() {
                counter.entry(part).and_modify(|c| *c += 1).or_insert(1);
            }
        });

    counter
}
