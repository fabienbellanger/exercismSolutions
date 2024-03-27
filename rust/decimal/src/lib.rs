const BASE: Digit = 10;

type Digit = u8;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, Default)]
pub struct Decimal {
    pub negative: bool,
    pub exponent: i32,
    pub digits: Vec<Digit>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.is_empty() {
            return None;
        }

        let number = Decimal::default();
        dbg!(&number);

        let (before, after) = input.split_once('.')?;
        let decimals = after.len();
        dbg!(before, after, decimals);

        todo!("Create a new decimal with a value of {input}")
    }
}
