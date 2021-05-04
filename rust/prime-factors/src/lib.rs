pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 2;
    let mut r = n;

    while r != 1 {
        if r % i == 0 {
            result.push(i);
            r /= i;
        } else {
            i += 1;
        }
    }

    result
}
