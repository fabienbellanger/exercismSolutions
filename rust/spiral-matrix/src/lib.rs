enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Init matrix
    let mut spiral = Vec::with_capacity(size as usize);
    for i in 0..size {
        spiral[i as usize] = Vec::with_capacity(size as usize);
    }

    dbg!(&spiral);

    spiral
}
