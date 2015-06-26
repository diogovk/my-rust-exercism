
#[derive(Clone, PartialEq, Debug)]
// each element is associated with the bitmask which represents it
pub enum Allergen {
  Eggs = 1,
  Peanuts = 2,
  Shellfish = 4,
  Strawberries = 8,
  Tomatoes = 16,
  Chocolate = 32,
  Pollen = 64,
  Cats = 128
}

impl Allergen {
  pub fn all() -> Vec<Allergen>{
      vec![Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries,
                Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats]
  }
}

pub struct Allergies(pub u32) ;

impl Allergies {
  pub fn is_allergic_to(&self, thing: &Allergen)  -> bool{
    (thing.clone() as u32 & self.0) != 0
  }

  pub fn allergies(&self) -> Vec<Allergen> {
      let mut allergens = Allergen::all();
      allergens.retain(|ref thing| self.is_allergic_to(thing));
      allergens
  }

}

