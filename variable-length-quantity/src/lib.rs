#[derive(Debug)]
pub enum Error {
    EncodedNumberIncomplete,
    OverflowDuringDecoding,
}

struct VLQBytes {
    number: u32,
    remaining_bytes: u8,
}

impl VLQBytes {
    fn new(number: &u32) -> VLQBytes {
        let significant_bits = 32 - number.leading_zeros() as u8;
        VLQBytes {
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

impl Iterator for VLQBytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_bytes > 1 {
            let bits = (self.number >> (self.remaining_bytes - 1) * 7) & 0b01111111;
            self.remaining_bytes -= 1;
            Some((bits | 0b10000000) as u8)
        } else if self.remaining_bytes == 1 {
            self.remaining_bytes -= 1;
            Some((self.number & 0b01111111) as u8)
        } else {
            None
        }
    }
}

fn vlq_has_more_bytes(byte: &u8) -> bool {
    (byte & 0b10000000) == 0b10000000
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(VLQBytes::new).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    match bytes.last() {
        None => Ok(vec![]),
        Some(last_byte) if vlq_has_more_bytes(last_byte) => Err(Error::EncodedNumberIncomplete),
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

                if vlq_has_more_bytes(byte) {
                    current_value += (byte ^ 0b10000000) as u32;
                } else {
                    current_value += *byte as u32;
                    values.push(current_value);
                    current_value = 0;
                }
            }
            Ok(values)
        }
    }
}
