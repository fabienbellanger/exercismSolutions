#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let n = items.len();
    let mut t = vec![vec![0; max_weight as usize + 1]; n + 1];

    for c in 0..=max_weight {
        t[0][c as usize] = 0;
    }

    for i in 1..=n {
        for c in 0..=max_weight {
            if c >= items[i - 1].weight {
                t[i][c as usize] = t[i - 1][c as usize]
                    .max(t[i - 1][(c - items[i - 1].weight) as usize] + items[i - 1].value);
            } else {
                t[i][c as usize] = t[i - 1][c as usize];
            }
        }
    }

    t[n][max_weight as usize]
}
