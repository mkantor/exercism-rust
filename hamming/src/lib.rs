pub fn hamming_distance(strand1: &str, strand2: &str) -> Result<usize, &'static str> {
    if strand1.len() != strand2.len() {
        Err("Strands must be of equal length")
    } else {
        Ok({
            let base_pair_iter = strand1.chars().zip(strand2.chars());
            base_pair_iter.filter(|&(base1, base2)| {
                base1 != base2
            }).count()
        })
    }
}
