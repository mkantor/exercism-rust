use std::collections::HashSet;

pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let mut non_primes = HashSet::new();
    for num in 2..(limit / 2) {
        if non_primes.contains(&num) {
            continue;
        } else {
            let mut multiple = num;
            while multiple + num <= limit {
                multiple += num;
                non_primes.insert(multiple);
            }
        }
    }

    let mut primes = vec![];
    for num in 2..(limit + 1) {
        if !non_primes.contains(&num) {
            primes.push(num);
        }
    }

    primes
}
