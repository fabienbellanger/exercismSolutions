use std::convert::TryFrom;

pub fn is_armstrong_number(num: u32) -> bool {
    let numbers: Vec<u32> = num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap())
        .collect();
    let numbers_length = numbers.len();
    let result: u64 = numbers
        .iter()
        .map(|i| i.pow(numbers_length as u32) as u64)
        .sum();

    match u32::try_from(result) {
        Ok(result) => result == num,
        _ => false,
    }
}
