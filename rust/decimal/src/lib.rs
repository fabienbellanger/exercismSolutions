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
        let input = input.trim_end_matches('0');

        // Find sign
        if input.starts_with('-') {
            number.negative = true;
        }

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
                number.exponent = point_index as i32 - first_non_zero as i32 - 1;
            }
            _ => number.exponent = 0, // TODO: fix
        }

        Some(number)
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
    }
}
