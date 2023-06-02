pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let _allergen_score = 2_u32.pow(*allergen as u32);
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
