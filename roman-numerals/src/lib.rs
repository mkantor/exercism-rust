#[derive(Clone, Copy)]
enum RomanDigit {
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}
impl RomanDigit {
    fn largest_fit(num: u16) -> Option<RomanDigit> {
        match num {
            n if n >= RomanDigit::M.into() => Some(RomanDigit::M),
            n if n >= RomanDigit::CM.into() => Some(RomanDigit::CM),
            n if n >= RomanDigit::D.into() => Some(RomanDigit::D),
            n if n >= RomanDigit::CD.into() => Some(RomanDigit::CD),
            n if n >= RomanDigit::C.into() => Some(RomanDigit::C),
            n if n >= RomanDigit::XC.into() => Some(RomanDigit::XC),
            n if n >= RomanDigit::L.into() => Some(RomanDigit::L),
            n if n >= RomanDigit::XL.into() => Some(RomanDigit::XL),
            n if n >= RomanDigit::X.into() => Some(RomanDigit::X),
            n if n >= RomanDigit::IX.into() => Some(RomanDigit::IX),
            n if n >= RomanDigit::V.into() => Some(RomanDigit::V),
            n if n >= RomanDigit::IV.into() => Some(RomanDigit::IV),
            n if n >= RomanDigit::I.into() => Some(RomanDigit::I),
            _ => None,
        }
    }
}
impl From<RomanDigit> for u16 {
    fn from(digit: RomanDigit) -> u16 {
        match digit {
            RomanDigit::I => 1,
            RomanDigit::IV => 4,
            RomanDigit::V => 5,
            RomanDigit::IX => 9,
            RomanDigit::X => 10,
            RomanDigit::XL => 40,
            RomanDigit::L => 50,
            RomanDigit::XC => 90,
            RomanDigit::C => 100,
            RomanDigit::CD => 400,
            RomanDigit::D => 500,
            RomanDigit::CM => 900,
            RomanDigit::M => 1000,
        }
    }
}
impl ToString for RomanDigit {
    fn to_string(&self) -> String {
        match *self {
                RomanDigit::I => "I",
                RomanDigit::IV => "IV",
                RomanDigit::V => "V",
                RomanDigit::IX => "IX",
                RomanDigit::X => "X",
                RomanDigit::XL => "XL",
                RomanDigit::L => "L",
                RomanDigit::XC => "XC",
                RomanDigit::C => "C",
                RomanDigit::CD => "CD",
                RomanDigit::D => "D",
                RomanDigit::CM => "CM",
                RomanDigit::M => "M",
            }
            .to_string()
    }
}

#[derive(Clone, Copy)]
struct RomanDigits(u16);
impl Iterator for RomanDigits {
    type Item = RomanDigit;
    fn next(&mut self) -> Option<RomanDigit> {
        let next = RomanDigit::largest_fit(self.0);
        if let Some(digit) = next {
            self.0 -= u16::from(digit);
        }
        next
    }
}

pub struct Roman(RomanDigits);
impl From<u16> for Roman {
    fn from(num: u16) -> Self {
        Roman(RomanDigits(num))
    }
}
impl ToString for Roman {
    fn to_string(&self) -> String {
        self.0.map(|digit| digit.to_string()).collect()
    }
}
