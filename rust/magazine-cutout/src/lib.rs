use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();

    for word in magazine {
        let entry = words.entry(*word).or_insert(0);
        *entry += 1;
    }

    for word in note {
        let w = words.entry(*word).or_insert(0);
        match w {
            0 => return false,
            _ => *w -= 1,
        }
    }

    true
}
