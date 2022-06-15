use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, IntEnum, Clone, Copy, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut values = all::<ResistorColor>()
        .map(color_to_value)
        .collect::<Vec<usize>>();

    values.sort_unstable();

    values
        .into_iter()
        .flat_map(|value| ResistorColor::from_int(value).ok())
        .collect::<Vec<_>>()
}
