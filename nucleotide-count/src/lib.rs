use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Nucleotide { A, C, G, T }
impl Nucleotide {
    // TODO: TryFrom would be better (but it's unstable).
    fn new(letter: char) -> Result<Nucleotide, &'static str> {
        match letter {
            'A' => Ok(Nucleotide::A),
            'C' => Ok(Nucleotide::C),
            'G' => Ok(Nucleotide::G),
            'T' => Ok(Nucleotide::T),
            _ => Err("Invalid DNA nucleotide"),
        }
    }
}

struct NucleotideCounts { a: usize, c: usize, g: usize, t: usize }
impl NucleotideCounts {
    // TODO: TryFrom would be better (but it's unstable).
    fn new(strand: &str) -> Result<Self, &str> {
        let mut counts = NucleotideCounts { a: 0, c: 0, g: 0, t: 0 };
        for letter in strand.chars() {
            match Nucleotide::new(letter)? {
                Nucleotide::A => counts.a += 1,
                Nucleotide::C => counts.c += 1,
                Nucleotide::G => counts.g += 1,
                Nucleotide::T => counts.t += 1,
            }
        }
        Ok(counts)
    }

    fn count(self: Self, n: Nucleotide) -> usize {
        match n {
            Nucleotide::A => self.a,
            Nucleotide::C => self.c,
            Nucleotide::G => self.g,
            Nucleotide::T => self.t,
        }
    }
}
impl From<NucleotideCounts> for HashMap<char, usize> {
    fn from(counts: NucleotideCounts) -> Self {
        let mut h = HashMap::new();
        h.insert('A', counts.a);
        h.insert('C', counts.c);
        h.insert('G', counts.g);
        h.insert('T', counts.t);
        h
    }
}

pub fn count(letter: char, strand: &str) -> Result<usize, &str> {
    Nucleotide::new(letter).and_then(|nucleotide| {
        Ok(NucleotideCounts::new(strand)?.count(nucleotide))
    })
}

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char, usize>, &str> {
    Ok(HashMap::from(NucleotideCounts::new(strand)?))
}
