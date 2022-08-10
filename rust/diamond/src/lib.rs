use std::ops::RangeInclusive;

const START: u8 = b'A';

pub fn get_diamond(c: char) -> Vec<String> {
    let range = RangeInclusive::new(0, c as u8 - START);


    unimplemented!(
        "Return the vector of strings which represent the diamond with particular char {}",
        c
    );
}
