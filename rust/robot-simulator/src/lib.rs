use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        let d = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self::new(self.x, self.y, d)
    }

    pub fn turn_left(self) -> Self {
        let d = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Self::new(self.x, self.y, d)
    }

    pub fn advance(self) -> Self {
        match self.direction {
            North => Self::new(self.x, self.y + 1, self.direction),
            East => Self::new(self.x + 1, self.y, self.direction),
            South => Self::new(self.x, self.y - 1, self.direction),
            West => Self::new(self.x - 1, self.y, self.direction),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
