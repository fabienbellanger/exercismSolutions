#[derive(Debug, PartialEq)]
pub enum Comparison {
    // A = [1, 2, 3], B = [1, 2, 3], A is equal to B
    Equal,

    // A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
    Sublist,

    // A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
    Superlist,
    
    // A = [1, 2, 4], B = [1, 2, 3, 4, 5], A and B are unequal
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
