/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn
        .chars()
        .filter(|c| c.is_numeric() || *c == 'X')
        .collect();

    if isbn.len() != 10 {
        return false;
    }
    if isbn.contains('X') && !isbn.ends_with('X') {
        return false;
    }

    let sum: usize = isbn
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            'X' => 10 * (10 - i),
            _ => c.to_string().parse::<usize>().unwrap() * (10 - i),
        })
        .sum();

    sum % 11 == 0
}
