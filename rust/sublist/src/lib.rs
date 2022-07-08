use std::fmt::Debug;

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

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    // Equal
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    // Sublist
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    if first_list_len < second_list_len {
        let is_sublist = second_list.windows(first_list_len).any(|l| l == first_list);
        if is_sublist {
            return Comparison::Sublist;
        }
    }

    // Superlist
    if second_list.is_empty() {
        return Comparison::Superlist;
    }
    if second_list_len < first_list_len {
        let is_superlist = first_list
            .windows(second_list_len)
            .any(|l| l == second_list);
        if is_superlist {
            return Comparison::Superlist;
        }
    }

    // Unequal
    Comparison::Unequal
}
