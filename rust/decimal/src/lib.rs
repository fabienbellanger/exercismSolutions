use std::{cmp::Ordering, ops::Add};

const BASE: Digit = 10;

type Digit = u8;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, Default)]
pub struct Decimal {
    pub negative: bool,
    pub exponent: i32,
    pub digits: Vec<Digit>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.is_empty() {
            return None;
        }

        let mut number = Decimal::default();

        // Remove zeros at the end
        let mut input = input.to_string();

        // Find sign
        if input.starts_with('-') {
            number.negative = true;
        }

        // Find .
        if !input.contains('.') {
            input = format!("{}.", input);
        }

        // Remove zeros at the end
        input = input.trim_end_matches('0').to_string();

        let mut point_index = None;
        let mut first_non_zero = None;
        let mut j = 0;
        for d in input.chars() {
            if d == '-' || d == '+' {
                continue;
            } else if d == '.' {
                point_index = Some(j);
                j -= 1;
            } else {
                let digit = d.to_digit(BASE as u32)?;

                if digit != 0 && first_non_zero.is_none() {
                    first_non_zero = Some(j);
                }

                if first_non_zero.is_some() {
                    number.digits.push(digit as Digit);
                }
            }
            j += 1;
        }

        match (point_index, first_non_zero) {
            (Some(point_index), Some(first_non_zero)) => {
                number.exponent = point_index - first_non_zero - 1;
            }
            _ => {
                number.exponent = 0;
                number.digits = vec![0];
            }
        }

        Some(number)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative
            && self.exponent == other.exponent
            && self.digits == other.digits
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.negative != other.negative {
            return Some(match self.negative {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
        }

        if self.exponent != other.exponent {
            return Some(self.exponent.cmp(&other.exponent));
        }

        if self.digits.len() != other.digits.len() {
            return Some(self.digits.len().cmp(&other.digits.len()));
        }

        for (a, b) in self.digits.iter().zip(other.digits.iter()) {
            if a != b {
                return Some(a.cmp(b));
            }
        }

        Some(Ordering::Equal)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Decimal::default();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_try_from() {
        let number = Decimal::try_from("3.14").unwrap();
        assert_eq!(number.digits, vec![3, 1, 4]);
        assert_eq!(number.exponent, 0);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("13.014").unwrap();
        assert_eq!(number.digits, vec![1, 3, 0, 1, 4]);
        assert_eq!(number.exponent, 1);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("-0.014").unwrap();
        assert_eq!(number.digits, vec![1, 4]);
        assert_eq!(number.exponent, -2);
        assert_eq!(number.negative, true);

        let number = Decimal::try_from("+230.400").unwrap();
        assert_eq!(number.digits, vec![2, 3, 0, 4]);
        assert_eq!(number.exponent, 2);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("0.0").unwrap();
        assert_eq!(number.digits, vec![0]);
        assert_eq!(number.exponent, 0);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("0").unwrap();
        assert_eq!(number.digits, vec![0]);
        assert_eq!(number.exponent, 0);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("0000000023").unwrap();
        assert_eq!(number.digits, vec![2, 3]);
        assert_eq!(number.exponent, 1);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("2300").unwrap();
        assert_eq!(number.digits, vec![2, 3, 0, 0]);
        assert_eq!(number.exponent, 3);
        assert_eq!(number.negative, false);

        let number = Decimal::try_from("");
        assert!(number.is_none());
    }
}
