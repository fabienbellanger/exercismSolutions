use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let word_set = ordered_list(word);
    let word_length = word.len();
    let word_to_uppercase = to_uppercase(word);

    for item in possible_anagrams {
        if item.len() == word_length {
            let item_set = ordered_list(item);
            if item_set == word_set && word_to_uppercase != to_uppercase(*item) {
                result.insert(item);
            }
        }
    }

    result
}

fn word_iterator(word: &'_ str) -> impl Iterator<Item = String> + '_ {
    word.chars().map(|c| {
        if c.is_lowercase() {
            c.to_uppercase().to_string()
        } else {
            c.to_string()
        }
    })
}

fn to_uppercase(word: &str) -> String {
    word_iterator(word).collect()
}

fn ordered_list(word: &str) -> Vec<String> {
    let mut ordered_word: Vec<String> = word_iterator(word).collect();

    ordered_word.sort_unstable();
    ordered_word
}
