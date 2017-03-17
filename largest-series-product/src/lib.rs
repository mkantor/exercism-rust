#[derive(Debug)]
pub enum LSPError {
    NotADigit,
    InvalidLength,
}

pub fn lsp(digits: &str, length: usize) -> Result<u32, LSPError> {
    if length == 0 {
        Ok(1)
    } else if length > digits.len() {
        Err(LSPError::InvalidLength)
    } else {
        let mut max_product = 0;
        for series in digits.as_bytes().windows(length) {
            let mut product = 1;
            for byte in series {
                product *= (*byte as char).to_digit(10).ok_or(LSPError::NotADigit)?;
            }
            if product > max_product {
                max_product = product;
            }
        }
        Ok(max_product)
    }
}
