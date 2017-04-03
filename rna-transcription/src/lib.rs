#[derive(Debug)]
pub enum Error {
    InvalidNucleotide,
}

pub trait Nucleotide: Sized {
    fn new(letter: char) -> Result<Self, Error>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum DnaNucleotide {
    A,
    C,
    G,
    T,
}
impl Nucleotide for DnaNucleotide {
    fn new(letter: char) -> Result<Self, Error> {
        match letter {
            'A' => Ok(DnaNucleotide::A),
            'C' => Ok(DnaNucleotide::C),
            'G' => Ok(DnaNucleotide::G),
            'T' => Ok(DnaNucleotide::T),
            _ => Err(Error::InvalidNucleotide),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RnaNucleotide {
    A,
    C,
    G,
    U,
}
impl Nucleotide for RnaNucleotide {
    fn new(letter: char) -> Result<Self, Error> {
        match letter {
            'A' => Ok(RnaNucleotide::A),
            'C' => Ok(RnaNucleotide::C),
            'G' => Ok(RnaNucleotide::G),
            'U' => Ok(RnaNucleotide::U),
            _ => Err(Error::InvalidNucleotide),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct NucleicAcid<T: Nucleotide> {
    chain: Vec<T>,
}
impl<T: Nucleotide> NucleicAcid<T> {
    pub fn new(strand: &str) -> Self {
        // This sucks, but the tests require a constructor that doesn't return errors (even though
        // this can clearly fail).
        Self::safe_new(strand).unwrap()
    }

    fn safe_new(strand: &str) -> Result<Self, Error> {
        let mut acid = NucleicAcid { chain: Vec::with_capacity(strand.len()) };
        for letter in strand.chars() {
            acid.chain.push(T::new(letter)?);
        }
        Ok(acid)
    }
}

impl<T: Nucleotide> IntoIterator for NucleicAcid<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.chain.into_iter()
    }
}

use std::iter::FromIterator;
impl FromIterator<DnaNucleotide> for NucleicAcid<RnaNucleotide> {
    fn from_iter<I: IntoIterator<Item = DnaNucleotide>>(iter: I) -> Self {
        NucleicAcid {
            chain: iter.into_iter()
                .map(|nucleotide| match nucleotide {
                         DnaNucleotide::G => RnaNucleotide::C,
                         DnaNucleotide::C => RnaNucleotide::G,
                         DnaNucleotide::T => RnaNucleotide::A,
                         DnaNucleotide::A => RnaNucleotide::U,
                     })
                .collect(),
        }
    }
}

pub type RibonucleicAcid = NucleicAcid<RnaNucleotide>;
pub type DeoxyribonucleicAcid = NucleicAcid<DnaNucleotide>;
impl DeoxyribonucleicAcid {
    pub fn to_rna(self) -> RibonucleicAcid {
        self.into_iter().collect()
    }
}
