pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        let mut number = 0;
        for (i, character) in self.chars().filter(|&c| c != ' ').enumerate() {
            match character.to_digit(10) {
                Some(digit) => {
                    let multiplier = 10u64.pow(i as u32);
                    number += multiplier * digit as u64;
                }
                None => return false,
            }
        }
        number.valid_luhn()
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}

macro_rules! uint_impl {
    ($t: ty) => {
        impl Luhn for $t {
            fn valid_luhn(&self) -> bool {
                // TODO? Could avoid widening integers by putting the iteration/validation inline
                // here and ditching the separate LuhnNumber struct.
                LuhnNumber(*self as u64).is_valid()
            }
        }
    }
}
uint_impl! { u8 }
uint_impl! { u16 }
uint_impl! { u32 }
uint_impl! { u64 }
uint_impl! { usize }


struct LuhnNumber(u64);

/// Iterate over digits from least to most significant.
impl Iterator for LuhnNumber {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            LuhnNumber(number) if number == 0 => None,
            LuhnNumber(ref mut number) => {
                let digit = (*number % 10) as u8;
                *number /= 10;
                Some(digit)
            }
        }
    }
}

impl LuhnNumber {
    fn is_valid(self) -> bool {
        if self.0 < 10 {
            false
        } else {
            self.enumerate()
                .map(|(i, digit)| match i % 2 {
                         0 => digit as u64,
                         _ => {
                             match digit * 2 {
                                 output @ 0...9 => output as u64,
                                 output => (output - 9) as u64,
                             }
                         }
                     })
                .sum::<u64>() % 10 == 0
        }
    }
}
