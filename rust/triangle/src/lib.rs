use std::fmt::Debug;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct Triangle<T> 
where T: Debug + PartialOrd + Default {
    sides: [T; 3],
}

impl<T> Triangle<T>
where T: Debug + PartialOrd + Default {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        dbg!(T::default());
        // if sides.iter().any(|s|  (*s).partial_cmp(&T::default()).is_some()) {
        //     return None;
        // }
        // Some(Self{
        //     sides
        // })
        None
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
