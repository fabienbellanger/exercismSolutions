use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..=sum / 3 {
        for b in a + 1..sum - a {
            let c = sum - a - b;
            if a + b + c == sum && a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }

    triplets
}
