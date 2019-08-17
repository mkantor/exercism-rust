pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 {
        None
    } else {
        let mut current_n = n;
        let mut steps = 0;
        while current_n != 1 {
            if current_n % 2 == 0 {
                current_n = current_n / 2;
            } else {
                current_n = current_n * 3 + 1;
            }
            steps += 1;
        }
        Some(steps)
    }
}
