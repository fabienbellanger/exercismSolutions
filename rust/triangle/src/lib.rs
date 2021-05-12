use std::cmp::Ordering;
use std::fmt::Debug;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct Triangle<T> 
where T: Debug + PartialOrd {
    sides: [T; 3],
}

impl<T> Triangle<T>
where T: Debug + PartialOrd {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|s|  (*s).partial_cmp(&0).is_some()) {

        }
    }

    pub fn is_equilateral(&self) -> bool {
        unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
