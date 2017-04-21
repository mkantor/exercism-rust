use Digit::*;
use LuhnData::*;

#[derive(Clone, Copy)]
enum Digit {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}
impl Digit {
    // TryFrom would be better, but is unstable.
    fn from_char(input: char) -> Option<Self> {
        match input {
            '0' => Some(_0),
            '1' => Some(_1),
            '2' => Some(_2),
            '3' => Some(_3),
            '4' => Some(_4),
            '5' => Some(_5),
            '6' => Some(_6),
            '7' => Some(_7),
            '8' => Some(_8),
            '9' => Some(_9),
            _ => None,
        }
    }
}
impl From<Digit> for u8 {
    fn from(input: Digit) -> u8 {
        match input {
            _0 => 0,
            _1 => 1,
            _2 => 2,
            _3 => 3,
            _4 => 4,
            _5 => 5,
            _6 => 6,
            _7 => 7,
            _8 => 8,
            _9 => 9,
        }
    }
}

enum LuhnData {
    BadInput,
    GoodInput { digits: Vec<Digit> },
}

pub struct Luhn {
    data: LuhnData,
}
impl Luhn {
    pub fn is_valid(&self) -> bool {
        match self.data {
            BadInput => false,
            GoodInput { ref digits } if digits.len() <= 1 => false,
            GoodInput { ref digits } => {
                let mut sum = 0;
                for (i, digit) in digits.iter().rev().enumerate() {
                    let mut num = u8::from(*digit);
                    if i % 2 != 0 {
                        num *= 2;
                    }
                    if num > 9 {
                        num -= 9;
                    }
                    sum += num;
                }
                sum % 10 == 0
            }
        }
    }
}
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        // FIXME? I could avoid allocating a Vec here, instead just holding the &str and using its
        // chars() Iterator directly in is_valid(). The downside of that approach is that Luhn
        // instances would need to be lifetime-coupled to the input &str.
        let mut digits = vec![];
        for character in input.chars() {
            if character != ' ' {
                match Digit::from_char(character) {
                    Some(digit) => digits.push(digit),
                    None => return Luhn { data: BadInput },
                }
            }
        }
        Luhn { data: GoodInput { digits: digits } }
    }
}
impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}
macro_rules! from_impl_via_to_string {
    ($t:ty) => {
        impl From<$t> for Luhn {
            fn from(input: $t) -> Self {
                Luhn::from(input.to_string())
            }
        }
    }
}
// FIXME? Converting through String allocations for these types is wasteful (I could implement
// Iterators to emit Digits directly from the ints instead). However this has the advantage of
// being succinct and re-using what I already have.
from_impl_via_to_string! { u8 }
from_impl_via_to_string! { u16 }
from_impl_via_to_string! { u32 }
from_impl_via_to_string! { u64 }
from_impl_via_to_string! { usize }
