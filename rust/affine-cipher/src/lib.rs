const ALPHABET_LENGTH: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// The MMI of a is x such that the remainder after dividing ax by m is 1:
/// `ax mod m = 1`
fn find_mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|n| a * n % m == 1)
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, ALPHABET_LENGTH) {
        Some(_s) => Ok(encode_str(plaintext, a, b, ALPHABET_LENGTH)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

/// `E(x) = (a*i + b) mod m`
/// - i is the letter's index from 0 to the length of the alphabet - 1
/// - m is the length of the alphabet. For the Roman alphabet m is 26.
/// - a and b are integers which make the encryption key
fn encode_str(plaintext: &str, a: i32, b: i32, m: i32) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_digit() {
            true => c,
            false => ((a * (c as i32 - 'a' as i32) + b).rem_euclid(m) + 'a' as i32) as u8 as char,
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, ALPHABET_LENGTH) {
        Some(mmi) => Ok(decode_str(ciphertext, b, ALPHABET_LENGTH, mmi)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

/// `D(y) = (a^-1)(y - b) mod m`
/// - y is the numeric value of an encrypted letter, i.e., y = E(x)
/// - it is important to note that a^-1 is the modular multiplicative inverse (MMI) of a mod m
/// - the modular multiplicative inverse only exists if a and m are coprime.
fn decode_str(ciphertext: &str, b: i32, m: i32, mmi: i32) -> String {
    ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_digit() {
            true => c,
            false => ((mmi * (c as i32 - 'a' as i32 - b)).rem_euclid(m) + 'a' as i32) as u8 as char,
        })
        .collect::<String>()
}
