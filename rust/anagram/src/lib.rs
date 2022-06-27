use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let word_set: Vec<char> = order(word);

    for item in possible_anagrams {
        let item_set = order(item);
        if item_set == word_set && to_uppercase(word) != to_uppercase(*item) {
            result.insert(item);
        }
    }

    result
}

fn to_uppercase(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'α' => 'Α',
            'β' => 'Β',
            'γ' => 'Γ',
            _ => {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            }
        })
        .collect()
}

fn order(word: &str) -> Vec<char> {
    let mut ordered_word: Vec<char> = word
        .chars()
        .map(|c| match c {
            'α' => 'Α',
            'β' => 'Β',
            'γ' => 'Γ',
            _ => {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            }
        })
        .collect();

    ordered_word.sort_unstable();
    ordered_word
}
