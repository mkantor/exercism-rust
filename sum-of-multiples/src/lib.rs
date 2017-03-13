use std::collections::HashSet;

struct Multiples {
    limit: u64,
    number: u64,
    current_multiple: u64,
}

impl Multiples {
    pub fn new(limit: u64, number: u64) -> Multiples {
        Multiples {
            limit: limit,
            number: number,
            current_multiple: number,
        }
    }
}

impl Iterator for Multiples {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current_multiple < self.limit {
            let multiple = self.current_multiple;
            self.current_multiple += self.number;
            Some(multiple)
        } else {
            None
        }
    }
}

pub fn sum_of_multiples(limit: u64, numbers: &Vec<u64>) -> u64 {
    // Keep track of multiples we've already seen to avoid dupes.
    // For example, when limit = 11 and numbers = &vec![2, 5], don't add 10 twice.
    let mut seen_multiples: HashSet<u64> = HashSet::new();

    numbers.iter().fold(0, |sum, &number| {
        sum + Multiples::new(limit, number).filter(|&multiple| {
            seen_multiples.insert(multiple)
        }).sum::<u64>()
    })
}
