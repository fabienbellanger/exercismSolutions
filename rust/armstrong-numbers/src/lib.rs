pub fn is_armstrong_number(num: u32) -> bool {
    let numbers: Vec<u32> = num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap())
        .collect();
    let numbers_length = numbers.len();
    let result: u32 = numbers.iter().map(|i| i.pow(numbers_length as u32)).sum();
    result == num
}
