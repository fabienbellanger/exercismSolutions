const A_UPPER: u8 = b'A';
const A_LOWER: u8 = b'a';
const LENGTH: u8 = 26;

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c.is_ascii_alphabetic() {
            false => c,
            true => {
                let m = ((LENGTH as i8 + key) % LENGTH as i8) as u8;

                match c.is_ascii_lowercase() {
                    true => char::from(A_LOWER + (c as u8 - A_LOWER + m) % LENGTH),
                    false => char::from(A_UPPER + (c as u8 - A_UPPER + m) % LENGTH),
                }
            }
        })
        .collect()
}
