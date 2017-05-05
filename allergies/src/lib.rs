macro_rules! allergens {
    ($($name:ident = $value:expr),* $(,)*) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Allergen {
            $($name = $value),*,
        }

        impl Allergen {
            fn all() -> &'static [Allergen] {
                static ALL: &'static [Allergen] = &[$(Allergen::$name),*];
                ALL
            }
        }
    }
}

allergens! {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies {
    score: usize,
}

impl Allergies {
    pub fn new(allergy_score: usize) -> Self {
        Allergies { score: allergy_score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = *allergen as usize;
        self.score & allergen_score == allergen_score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .cloned()
            .collect()
    }
}
