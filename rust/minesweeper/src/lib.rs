use std::collections::HashSet;

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mines = get_mines(minefield);

    let mut grid = vec![];
    for (y, line) in minefield.iter().enumerate() {
        let mut new_line = String::new();

        for (x, c) in line.chars().enumerate() {
            if !mines.contains(&(x, y)) {
                let mut n = 0;
                for dir in DIRECTIONS {
                    let new_x = (x as isize + dir.0) as usize;
                    let new_y = (y as isize + dir.1) as usize;

                    if mines.contains(&(new_x, new_y)) {
                        n += 1;
                    }
                }

                if n == 0 {
                    new_line.push(' ');
                } else {
                    let s = n.to_string();
                    new_line.push_str(s.as_str());
                }
            } else {
                new_line.push(c);
            }
        }

        grid.push(new_line);
    }

    grid
}

fn get_mines(minefield: &[&str]) -> HashSet<(usize, usize)> {
    minefield
        .iter()
        .enumerate()
        .flat_map(|(y, lines)| {
            lines
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(_x, b)| **b == 42) // => *
                .map(|(x, _b)| (x, y))
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect::<HashSet<(usize, usize)>>()
}
