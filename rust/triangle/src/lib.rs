use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(Debug)]
pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Debug + Display + PartialOrd + PartialEq + Default + Copy + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|s| s <= &T::default()) {
            return None;
        }

        // Valid triangle ?
        if (sides[0] > sides[1] + sides[2])
            || (sides[1] > sides[0] + sides[2])
            || (sides[2] > sides[0] + sides[1])
        {
            return None;
        }

        Some(Self { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides_number() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides_number() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides_number() == 2
    }

    fn sides_number(&self) -> usize {
        self.sides
            .iter()
            .map(|side| (*side).to_string())
            .collect::<HashSet<String>>()
            .len()
    }
}
