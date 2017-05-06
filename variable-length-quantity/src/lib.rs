#[derive(Debug)]
pub enum Error {
    EncodedNumberIncomplete,
    OverflowDuringDecoding,
}

struct VLQEncoder {
    number: u32,
    remaining_bytes: u8,
}

impl VLQEncoder {
    fn new(number: &u32) -> Self {
        let significant_bits = 32 - number.leading_zeros() as u8;
        Self {
            number: *number,
            remaining_bytes: if *number == 0 {
                1
            } else if significant_bits % 7 != 0 {
                significant_bits / 7 + 1
            } else {
                significant_bits / 7
            },
        }
    }
}

impl Iterator for VLQEncoder {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_bytes > 1 {
            self.remaining_bytes -= 1;
            let bits = (self.number >> (self.remaining_bytes * 7)) & 0b0111_1111;
            Some((bits | 0b1000_0000) as u8)
        } else if self.remaining_bytes == 1 {
            self.remaining_bytes -= 1;
            Some((self.number & 0b0111_1111) as u8)
        } else {
            None
        }
    }
}

fn is_last_byte(byte: &u8) -> bool {
    (byte & 0b1000_0000) == 0
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(VLQEncoder::new).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    match bytes.last() {
        None => Ok(vec![]),
        Some(last_byte) if !is_last_byte(last_byte) => Err(Error::EncodedNumberIncomplete),
        Some(_) => {
            let mut values = vec![];
            let mut current_value = 0u32;
            for byte in bytes {
                if current_value > u32::max_value() >> 7 {
                    // current_value would overflow if it was shifted again.
                    return Err(Error::OverflowDuringDecoding);
                } else {
                    current_value <<= 7;
                }

                current_value |= (byte & 0b0111_1111) as u32;
                if is_last_byte(byte) {
                    values.push(current_value);
                    current_value = 0;
                }
            }
            Ok(values)
        }
    }
}
