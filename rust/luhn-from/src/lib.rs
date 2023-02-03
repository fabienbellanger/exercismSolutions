pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = self.code.replace(' ', "");

        if code.len() <= 1 || code.chars().any(|c| !c.is_ascii_digit()) {
            return false;
        }

        let sum: u16 = code
            .chars()
            .rev()
            .filter_map(|c| c.to_string().parse::<u16>().ok())
            .enumerate()
            .map(|(i, d)| {
                if i % 2 == 1 {
                    let double = d * 2;
                    if double > 9 {
                        double - 9
                    } else {
                        double
                    }
                } else {
                    d
                }
            })
            .sum();

        sum % 10 == 0
    }
}

impl<T: std::string::ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self { code: input.to_string() }
    }
}