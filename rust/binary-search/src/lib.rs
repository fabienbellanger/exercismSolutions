use std::cmp::Ord;

pub fn find<S: AsRef<[T]>, T: Ord>(array: S, key: T) -> Option<usize> {
    let mut sub = array.as_ref();
    let mut index = 0;

    while sub.len() >= 1 {
        if sub.len() == 1 && sub[0] != key {
            return None;
        }

        let middle = sub.len() / 2;
        if sub[middle] == key {
            return Some(index + middle);
        }

        let (left, right) = sub.split_at(middle);
        if key > sub[middle] {
            sub = right;
            index += middle;
        } else {
            sub = left;
        }
    }

    None
}
