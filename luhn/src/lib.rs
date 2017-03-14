pub fn is_valid(digits: &str) -> bool {
    // TODO: Could go for a more functional implementation. I tried that first but was having
    // trouble coming up with a straightforward way to do early return when encountering an
    // invalid character. Could try Result and `?` as I did in the nucleotide-count problem.

    let normalized_digits = digits.chars().filter(|&digit_or_space| {
        digit_or_space != ' '
    });

    let mut sum = 0;
    let mut count = 0; // FIXME: Can I eliminate this? How else can I handle inputs like " 0"?
    for (i, digit) in normalized_digits.rev().enumerate() {
        let double_this_digit = i % 2 != 0;
        // FIXME: Too much nesting in here.
        if let Some(num) = digit.to_digit(10) {
            if double_this_digit {
                let adjusted_num = num * 2;
                if adjusted_num > 9 {
                    sum += adjusted_num - 9;
                } else {
                    sum += adjusted_num;
                }
            } else {
                sum += num;
            }
        } else {
            // Digit is not numeric, so the input is invalid.
            return false;
        }
        count += 1;
    }

    if count <= 1 {
        false
    } else {
        sum % 10 == 0
    }
}
