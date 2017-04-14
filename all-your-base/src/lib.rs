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
    let number_of_digits = number_of_digits(number, base);
    let mut result = Vec::with_capacity(number_of_digits as usize);
    let mut remaining = number;
    let mut place = number_of_digits;
    while place > 0 {
        place -= 1;
        let multiplier = base.pow(place);
        let next_digit = remaining / multiplier;
        remaining -= next_digit * multiplier;
        result.push(next_digit);
    }
    Ok(result)
}

// FIXME: Is there a more direct way to calculate this?
fn number_of_digits(number: u32, base: u32) -> u32 {
    if number == 0 {
        0
    } else {
        let mut digits = 1;
        while base.pow(digits) <= number {
            digits += 1;
        }
        digits
    }
}
