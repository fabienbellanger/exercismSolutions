pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let nb_rows = input.len();
    let nb_cols = if input.is_empty() {
        0
    } else {
        input.first().unwrap_or(&vec![]).len()
    };

    let rows_max = get_rows_max(input);
    let cols_min = get_cols_min(input, nb_rows, nb_cols);

    let mut result = vec![];
    for (r, max) in rows_max.iter().enumerate().take(nb_rows) {
        for (c, min) in cols_min.iter().enumerate().take(nb_cols) {
            if input[r][c] == *max && input[r][c] == *min {
                result.push((r, c));
            }
        }
    }

    result
}

fn get_rows_max(input: &[Vec<u64>]) -> Vec<u64> {
    input
        .iter()
        .map(|row| *row.iter().max().unwrap_or(&0))
        .collect()
}

fn get_cols_min(input: &[Vec<u64>], nb_rows: usize, nb_cols: usize) -> Vec<u64> {
    let mut cols_min = Vec::new();

    for c in 0..nb_cols {
        let mut min = std::u64::MAX;
        for row in input.iter().take(nb_rows) {
            if row[c] < min {
                min = row[c];
            }
        }
        cols_min.push(min);
    }

    cols_min
}
