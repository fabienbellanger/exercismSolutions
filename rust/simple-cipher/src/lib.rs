use rand::prelude::*;

// TODO: Add a function simplify encode and decode functions
// fn transform(key: &str, s: &str, transform_char: fn(char, char) -> char) -> Option<String>

pub fn encode(key: &str, s: &str) -> Option<String> {
    match is_key_valid(key) {
        false => None,
        true => Some(
            s.chars()
                .zip(key.chars().cycle())
                .map(|(c, k)| encode_char(c, k))
                .collect::<String>(),
        ),
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    match is_key_valid(key) {
        false => None,
        true => Some(
            s.chars()
                .zip(key.chars().cycle())
                .map(|(c, k)| decode_char(c, k))
                .collect::<String>(),
        ),
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();

    let key = (0..rng.gen_range(100..200))
        .map(|_| char::from(rng.gen_range(97..=122)))
        .collect::<String>();

    (key.clone(), encode(&key, s).unwrap_or_default())
}

fn is_key_valid(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() && c.is_ascii_alphabetic())
}

fn encode_char(c: char, k: char) -> char {
    char::from((((c as u8) - 97 + (k as u8) - 97) % 26) + 97)
}

fn decode_char(c: char, k: char) -> char {
    char::from((((c as u8) + 26 - (k as u8)) % 26) + 97)
}
