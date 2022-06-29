use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mines = get_mines(minefield);

    dbg!(&mines);
    vec![]
}

fn get_mines(minefield: &[&str]) -> HashSet<(usize, usize)> {
    minefield
        .iter()
        .enumerate()
        .map(|(y, lines)| {
            lines
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(_x, b)| **b == 42) // => *
                .map(|(x, _b)| (x, y))
                .collect::<HashSet<(usize, usize)>>()
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>()
}
