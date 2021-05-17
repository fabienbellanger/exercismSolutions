pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut found = false;
    let mut sub = array;
    let mut index = 0;

    // let mut i = 0;
    while !found && !sub.is_empty() { // && i < 100 {
        let middle = sub.len() / 2;

        if sub[middle] == key {
            found = true;
            index += middle;
        } else {
            let (left, right) = sub.split_at(middle);
            
            if key > sub[middle] {
                sub = right;
                index += middle;
            } else {
                sub = left;
            }

            dbg!(middle, &left, &right, index);
            // DANGER : Infinite loop :
            // - identifies_that_a_value_is_not_included_in_the_array
            // - a_value_larger_than_the_arrays_largest_value_is_not_included()
        }

        // i += 1;
    }

    match found {
        true => Some(index),
        false => None,
    }
}
