pub fn find() -> Option<u32> {
    // Since it's given that the variables must sum to 1000, work backwards
    // from that fact.
    for a in 1..1000 {
        for b in 1..(1000 - a) {
            let c = 1000 - (a + b);
            if is_pythagorean_triplet(a, b, c) {
                return Some(a * b * c);
            }
        }
    }

    None
}

fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}
