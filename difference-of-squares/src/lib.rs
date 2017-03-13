pub fn square_of_sum(n: u64) -> u64 {
    (1..(n + 1)).sum::<u64>().pow(2)
}

pub fn sum_of_squares(n: u64) -> u64 {
    (1..(n + 1)).map(|m| m.pow(2)).sum()
}

pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}
