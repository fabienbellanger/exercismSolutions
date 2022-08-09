pub fn encrypt(input: &str) -> String {
    let input = normalize(input);
    let (rows, cols) = calculate_rows_cols(input.len());


    let square = input.chars()
        .collect::<Vec<_>>()
        .chunks(cols)
        .map(|chunk| format!("{line: <cols$}", line = chunk.iter().collect::<String>(), cols = cols))
        .collect::<Vec<String>>();
    dbg!(square);

    unimplemented!("Encrypt {:?} using a square code", input)
}

fn normalize(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_uppercase() {
            true => c.to_ascii_lowercase(),
            false => c
        })
        .collect()
}

fn calculate_rows_cols(length: usize) -> (usize, usize) {
    if length == 0 {
        return (0, 0);
    }

    let rows = (length as f64).sqrt() as usize;
    let cols = length / rows;

    (rows, match length % rows == 0 {
        true => cols,
        false => cols + 1,
    })
}
