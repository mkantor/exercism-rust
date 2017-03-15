#[derive(Debug)]
enum LuhnError {
    NotADigit,
    InvalidLength,
}

fn is_valid_result<I>(digits: I) -> Result<bool, LuhnError>
    where I: DoubleEndedIterator<Item=char>
{
    let mut sum = 0;
    let mut count = 0;
    for (i, digit) in digits.rev().enumerate() {
        let mut num = digit.to_digit(10).ok_or(LuhnError::NotADigit)?;
        if i % 2 != 0 {
            num *= 2;
        }
        if num > 9 {
            num -= 9;
        }
        sum += num;
        count += 1;
    }

    if count <= 1 {
        Err(LuhnError::InvalidLength)
    } else {
        Ok(sum % 10 == 0)
    }
}

pub fn is_valid(digits: &str) -> bool {
    is_valid_result(digits.chars().filter(|&digit_or_space| {
        digit_or_space != ' '
    })).unwrap_or(false)
}
