use phone_number::NANPPhoneNumber;
mod phone_number {
    use std::str::{FromStr, from_utf8};

    #[derive(Debug)]
    pub enum Error {
        TooManyDigits,
        NotEnoughDigits,
        InvalidCountryCode,
    }

    #[derive(PartialEq)]
    pub struct NANPPhoneNumber {
        // NOTE: Digits are stored as ASCII codepoints (not ints < 10).
        digits: [u8; 10],
    }

    impl NANPPhoneNumber {
        pub fn area_code(&self) -> &str {
            match from_utf8(&self.digits[0..3]) {
                Ok(digits) => digits,
                Err(_) => unreachable!("NANPPhoneNumber did not have enough digits"),
            }
        }

        pub fn exchange_code(&self) -> &str {
            match from_utf8(&self.digits[3..6]) {
                Ok(digits) => digits,
                Err(_) => unreachable!("NANPPhoneNumber did not have enough digits"),
            }
        }

        pub fn subscriber_number(&self) -> &str {
            match from_utf8(&self.digits[6..10]) {
                Ok(digits) => digits,
                Err(_) => unreachable!("NANPPhoneNumber did not have enough digits"),
            }
        }
    }

    impl FromStr for NANPPhoneNumber {
        type Err = Error;
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let digit_count = input.chars().filter(|c| c.is_digit(10)).count();
            let mut digit_chars = input.chars().filter(|c| c.is_digit(10));

            if digit_count > 11 {
                return Err(Error::TooManyDigits);
            } else if digit_count == 11 {
                // Note that this also advances the iterator to skip past the first digit.
                if digit_chars.next() != Some('1') {
                    return Err(Error::InvalidCountryCode);
                }
            }

            // There may be a nicer way to handle this using a macro (can macros do stuff N times
            // where N is a compile time const uint?).
            let mut d = digit_chars.map(|c| c as u8);
            match (d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next(),
                       d.next()) {
                    (Some(_1),
                     Some(_2),
                     Some(_3),
                     Some(_4),
                     Some(_5),
                     Some(_6),
                     Some(_7),
                     Some(_8),
                     Some(_9),
                     Some(_10)) => Ok([_1, _2, _3, _4, _5, _6, _7, _8, _9, _10]),
                    _ => Err(Error::NotEnoughDigits),
                }
                .map(|ten_digits| NANPPhoneNumber { digits: ten_digits })
        }
    }
}

pub fn number(input: &str) -> Option<String> {
    input
        .parse::<NANPPhoneNumber>()
        .map(|phone_number| {
                 format!("{}{}{}",
                         phone_number.area_code(),
                         phone_number.exchange_code(),
                         phone_number.subscriber_number())
             })
        .ok()
}

pub fn area_code(input: &str) -> Option<String> {
    input
        .parse::<NANPPhoneNumber>()
        .map(|phone_number| phone_number.area_code().to_string())
        .ok()
}

pub fn pretty_print(input: &str) -> String {
    match input.parse::<NANPPhoneNumber>() {
        Ok(phone_number) => {
            format!("({}) {}-{}",
                    phone_number.area_code(),
                    phone_number.exchange_code(),
                    phone_number.subscriber_number())
        }
        Err(_) => "invalid".to_string(),
    }
}
