use std::ops::RangeInclusive;

const ALPHA_RANGE: RangeInclusive<u8> = b'a'..=b'z';

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = transfom(plain, encode_valid_char);

    let plain: Vec<String> = plain
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|sl| sl.iter().collect::<String>())
        .collect();
    plain.join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transfom(cipher, decode_valid_char)
}

fn encode_valid_char(c: char) -> char {
    match ALPHA_RANGE.contains(&(c as u8)) {
        true => char::from(ALPHA_RANGE.end() - (c as u8 - ALPHA_RANGE.start())),
        false => c,
    }
}

fn decode_valid_char(c: char) -> char {
    match ALPHA_RANGE.contains(&(c as u8)) {
        true => char::from(ALPHA_RANGE.start() + (ALPHA_RANGE.end() - c as u8)),
        false => c,
    }
}

fn transfom(s: &str, transform_char: fn(char) -> char) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_lowercase() {
            true => c,
            false => c.to_ascii_lowercase(),
        })
        .map(transform_char)
        .collect()
}
