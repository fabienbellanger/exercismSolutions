// Given a number, find the sum of all the unique multiples of particular numbers up to but not including that number.
// If we list all the natural numbers below 20 that are multiples of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.
// The sum of these multiples is 78.

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|i| factors.iter().any(|f| *f != 0 && i % f == 0))
        .sum()
}
