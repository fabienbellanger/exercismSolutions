pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut not_primes = Vec::new();

    for n in 2..=upper_bound {
        if !not_primes.contains(&n) {
            primes.push(n);
            for m in n + 1..=upper_bound {
                if m % n == 0 {
                    not_primes.push(m);
                }
            }
        }
    }

    primes
}
