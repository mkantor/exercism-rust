use sieve::SieveStorage;

mod sieve {
    use std::ops::Range;

    pub struct SieveStorage {
        entries: Vec<bool>,
        start: usize,
    }

    pub struct Iter<'a> {
        data: &'a SieveStorage,
        current: usize,
    }

    impl SieveStorage {
        pub fn new(range: Range<usize>) -> Self {
            SieveStorage {
                entries: vec![true; range.len()],
                start: range.start,
            }
        }

        pub fn remove(&mut self, num: usize) {
            let index = num - self.start;
            if let Some(entry) = self.entries.get_mut(index) {
                *entry = false;
            }
        }

        pub fn contains(&self, num: usize) -> bool {
            let index = num - self.start;
            *self.entries.get(index).unwrap_or(&false)
        }

        pub fn iter(&self) -> Iter {
            Iter {
                current: self.start,
                data: self,
            }
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            while self.current < self.data.entries.len() + self.data.start {
                self.current += 1;
                if self.data.contains(self.current - 1) {
                    return Some(self.current - 1);
                }
            }
            None
        }
    }
}

pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let mut sieve = SieveStorage::new(2..(limit + 1));

    for num in 2..(limit / 2) {
        if !sieve.contains(num) {
            continue;
        } else {
            let mut multiple = num * 2;
            while multiple <= limit {
                sieve.remove(multiple);
                multiple += num;
            }
        }
    }

    sieve.iter().collect()
}
