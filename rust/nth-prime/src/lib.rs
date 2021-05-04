pub fn nth(n: u32) -> u32 {
    let mut primes = 0u32;
    let mut i = 2u32;

    loop {
        if is_prime(i) {
            if primes == n {
                break i;
            }
            primes += 1;
        }

        i += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    (2..n).all(|a| n % a != 0)
}
