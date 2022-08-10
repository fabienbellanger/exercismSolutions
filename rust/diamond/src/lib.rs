const START: u8 = b'A';

pub fn get_diamond(c: char) -> Vec<String> {
    let end = c as u8 - START;

    // First part
    // ----------
    //
    // 0 |       A
    // 1 |     B   B
    // 2 |   C       C
    // 3 | D           D
    // ------------------
    //   | 0 1 2 3 4 5 6

    let grid: Vec<String> = (0..=end)
        .map(|i| {
            (0..=end * 2)
                .map(|n| {
                    if n == end - i || n == end + i {
                        char::from(START + i)
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .collect();
    dbg!(&grid);

    // Second part
    // -----------
    //
    // 0 |       A
    // 1 |     B   B
    // 2 |   C       C
    // 3 | D           D
    // 4 |   C       C
    // 5 |     B   B
    // 6 |       A
    // ------------------
    //   | 0 1 2 3 4 5 6
    let mut _grid_2 = grid.clone();

    unimplemented!(
        "Return the vector of strings which represent the diamond with particular char {}",
        c
    );
}
