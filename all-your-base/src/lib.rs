use base::Base;

mod base {
    pub struct Base(u32);
    impl Base {
        pub fn new(input: u32) -> Option<Self> {
            if input <= 1 { None } else { Some(Base(input)) }
        }
    }
    impl From<Base> for u32 {
        fn from(base: Base) -> Self {
            base.0
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidBase,
    DigitDoesNotFitBase,
}

pub fn convert(number: &[u32], from_base_int: u32, to_base_int: u32) -> Result<Vec<u32>, Error> {
    let from_base = Base::new(from_base_int).ok_or(Error::InvalidBase)?;
    let to_base = Base::new(to_base_int).ok_or(Error::InvalidBase)?;
    to_int(number, from_base).map(|number_as_int| to_vec(number_as_int, to_base))
}

fn to_int(number: &[u32], base: Base) -> Result<u32, Error> {
    let base_int = u32::from(base);
    if number.iter().any(|&digit| digit >= base_int) {
        Err(Error::DigitDoesNotFitBase)
    } else {
        Ok(number.iter()
               .rev()
               .enumerate()
               .map(|(place, digit)| digit * base_int.pow(place as u32))
               .sum())
    }
}

fn to_vec(number: u32, base: Base) -> Vec<u32> {
    let base_int = u32::from(base);
    let mut digits = vec![];
    let mut remaining = number;
    while remaining > 0 {
        digits.push(remaining % base_int);
        remaining = remaining / base_int;
    }
    digits.reverse();
    digits
}
