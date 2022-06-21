use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Init matrix
    let mut spiral = vec![vec![0; size as usize]; size as usize];

    // Variables
    let mut added: HashSet<Point> = HashSet::new();
    let mut current: Option<Point> = None;
    let mut direction = Direction::Right;
    let mut new_x = 0;
    let mut new_y = 0;

    for y in 0..size {
        for x in 0..size {
            (direction, new_x, new_y) = get_direction(&current, &direction, &added, size);

            spiral[new_y as usize][new_x as usize] = size * y + x + 1;

            current = Some(Point::new(new_x, new_y));
            added.insert(Point::new(new_x, new_y));
        }
    }

    spiral
}

fn get_direction(
    current: &Option<Point>,
    direction: &Direction,
    added: &HashSet<Point>,
    size: u32,
) -> (Direction, u32, u32) {
    match (current, direction) {
        (None, _) => (*direction, 0, 0),
        (Some(current), direction) => match direction {
            Direction::Right => {
                let next = Point::new(current.x + 1, current.y);
                if next.x >= size || added.get(&next).is_some() {
                    (Direction::Down, current.x, current.y + 1)
                } else {
                    (*direction, current.x + 1, current.y)
                }
            }
            Direction::Down => {
                let next = Point::new(current.x, current.y + 1);
                if next.y >= size || added.get(&next).is_some() {
                    (Direction::Left, current.x - 1, current.y)
                } else {
                    (*direction, current.x, current.y + 1)
                }
            }
            Direction::Left => {
                if current.x == 0 {
                    (Direction::Up, current.x, current.y - 1)
                } else {
                    let next = Point::new(current.x - 1, current.y);
                    if added.get(&next).is_some() {
                        (Direction::Up, current.x, current.y - 1)
                    } else {
                        (*direction, current.x - 1, current.y)
                    }
                }
            }
            Direction::Up => {
                if current.y == 0 {
                    (Direction::Right, current.x + 1, current.y)
                } else {
                    let next = Point::new(current.x, current.y - 1);
                    if added.get(&next).is_some() {
                        (Direction::Right, current.x + 1, current.y)
                    } else {
                        (*direction, current.x, current.y - 1)
                    }
                }
            }
        },
    }
}
