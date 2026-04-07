fn neighbors(x: usize, y: usize, garden: &[&str]) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 {
                let row = garden.get(ny as usize);
                if let Some(row) = row {
                    if let Some(&cell) = row.as_bytes().get(nx as usize) {
                        if cell == b'*' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let garden = garden.to_vec();

    garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '*' {
                        '*'
                    } else {
                        let n = neighbors(x, y, &garden);
                        if n > 0 { (b'0' + n) as char } else { ' ' }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
