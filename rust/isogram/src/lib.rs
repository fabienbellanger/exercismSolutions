use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut used = HashSet::new();

    for c in candidate.chars() {
        if c.is_ascii_alphabetic() {
            let c = match c.is_ascii_uppercase() {
                true => c.to_ascii_lowercase(),
                false => c,
            };

            if used.contains(&c) {
                return false;
            } else {
                used.insert(c);
            }
        }
    }

    true
}
