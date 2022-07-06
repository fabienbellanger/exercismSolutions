/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut to_bytes: Vec<u32> = vec![];
        
        for d in value.to_string().chars() {
            match d.to_digit(10) {
                Some(d) => to_bytes.push(d),
                None => return None,
            }
        }

        let mut to_bytes_reversed = to_bytes.clone();
        to_bytes_reversed.reverse();
        
        if to_bytes == to_bytes_reversed {
            return Some(Self(value));
        }
        None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    unimplemented!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {} to {}",
        min, max
    );
}
