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
    let height = minefield.len() as isize;

    (0..height)
        .map(|y| {
            let width = minefield[y as usize].len() as isize;
            (0..width)
                .map(|x| {
                    if minefield[y as usize].as_bytes()[x as usize] == b'*' {
                        '*'
                    } else {
                        let count = DIRECTIONS
                            .iter()
                            .map(|(x_d, y_d)| (x + x_d, y + y_d))
                            .filter(|(x, y)| (0 <= *x && *x < width) && (0 <= *y && *y < height))
                            .filter(|(x, y)| minefield[*y as usize].as_bytes()[*x as usize] == b'*')
                            .count();
                        match count {
                            0 => ' ',
                            _ => (count as u8 + b'0') as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
