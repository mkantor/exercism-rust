use LuhnData::*;

enum LuhnData {
    BadInput,
    GoodInput { number: u64 },
}

/// Iterate over digits in GoodInput's number, going from least to most significant digit.
/// Sadly enum variants aren't types, otherwise I could just impl Iterator for GoodInput.
impl Iterator for LuhnData {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            BadInput => None,
            GoodInput { number } if number == 0 => None,
            GoodInput { ref mut number } => {
                let digit = *number % 10;
                *number /= 10;
                Some(digit)
            }
        }
    }
}

pub struct Luhn {
    data: LuhnData,
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        match self.data {
            BadInput => false,
            GoodInput { number } if number < 10 => false,
            data @ GoodInput { .. } => {
                data.enumerate()
                    .map(|(i, digit)| match i % 2 {
                             0 => digit,
                             _ => {
                                 match digit * 2 {
                                     output @ 0...9 => output,
                                     output => output - 9,
                                 }
                             }
                         })
                    .sum::<u64>() % 10 == 0
            }
        }
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        let mut number = 0;
        for (i, character) in input.chars().filter(|&c| c != ' ').enumerate() {
            match character.to_digit(10) {
                Some(digit) => {
                    // These casts are lame. Why doesn't Rust have things like this in std?
                    // impl Mul<u32> for u64 {
                    //     type Output = u64;
                    //     fn mul(self, rhs: u32) -> Self::Output {
                    //         self * rhs as u64
                    //     }
                    // }
                    // Also, I'm not sure why pow isn't part of Mul or in its own trait so it
                    // could be overloaded.
                    let multiplier = 10u64.pow(i as u32);
                    number += multiplier * digit as u64;
                }
                None => return Luhn { data: BadInput },
            }
        }
        Luhn { data: GoodInput { number: number } }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}

macro_rules! from_uint_impl {
    ($t: ty) => {
        impl From<$t> for Luhn {
            fn from(input: $t) -> Self {
                Luhn { data: GoodInput { number: input as u64 } }
            }
        }
    }
}
from_uint_impl! { u8 }
from_uint_impl! { u16 }
from_uint_impl! { u32 }
from_uint_impl! { u64 }
from_uint_impl! { usize }
