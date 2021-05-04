pub fn series(digits: &str, len: usize) -> Vec<String> {
    match (digits.len(), len) {
        (a, b) if a < b => vec![],
        _ => (0..digits.len() - len + 1)
            .map(|i| digits.get(i..i + len).unwrap().to_owned())
            .collect(),
    }
}
