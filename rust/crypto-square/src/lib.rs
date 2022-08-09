pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    let input = normalize(input);
    let width = calculate_width(input.len());

    let square = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(width)
        .map(|chunk| {
            format!(
                "{line: <width$}",
                line = chunk.iter().collect::<String>(),
                width = width
            )
        })
        .collect::<Vec<String>>();

    let mut transposed_square = vec!["".to_owned(); width];
    for row in square.iter() {
        for (c, col) in row.chars().enumerate() {
            transposed_square[c].push(col);
        }
    }

    transposed_square.join(" ")
}

fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_uppercase() {
            true => c.to_ascii_lowercase(),
            false => c,
        })
        .collect()
}

fn calculate_width(length: usize) -> usize {
    (length as f64).sqrt().ceil() as usize
}
