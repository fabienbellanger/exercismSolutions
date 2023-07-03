const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut sequence: Vec<&str> = ACTIONS
        .into_iter()
        .enumerate()
        .filter(|(i, _)| (n >> *i) & 1 == 1)
        .map(|(_, a)| a)
        .collect();

    if (n >> 4) & 1 == 1 {
        sequence.reverse();
    }

    sequence
}
