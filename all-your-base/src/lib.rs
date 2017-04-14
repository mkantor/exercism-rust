#[derive(Debug)]
pub enum Error {
    InvalidBase,
    DigitDoesNotFitBase,
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    to_int(number, from_base).and_then(|number_as_int| to_vec(number_as_int, to_base))
}

fn to_int(number: &[u32], base: u32) -> Result<u32, Error> {
    if base <= 1 {
        return Err(Error::InvalidBase);
    }
    let mut result = 0;
    let mut place = number.len() as u32;
    for digit in number {
        if *digit >= base {
            return Err(Error::DigitDoesNotFitBase);
        }
        place -= 1;
        result += digit * base.pow(place);
    }
    Ok(result)
}

fn to_vec(number: u32, base: u32) -> Result<Vec<u32>, Error> {
    if base <= 1 {
        return Err(Error::InvalidBase);
    }

    let mut digits = vec![];
    let mut remaining = number;
    while remaining > 0 {
        digits.push(remaining % base);
        remaining = remaining / base;
    }
    digits.reverse();
    Ok(digits)
}
