#![allow(unused)]

#[derive(Debug)]
pub struct Allergies {
    list: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,         //   1 = 2^0
    Peanuts,      //   2 = 2^1
    Shellfish,    //   4 = 2^2
    Strawberries, //   8 = 2^3
    Tomatoes,     //  16 = 2^4
    Chocolate,    //  32 = 2^5
    Pollen,       //  64 = 2^6
    Cats,         // 128 = 2^7
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // Max = 255 => All allergens
        let score = score % 256;

        let mut list = vec![];

        Self { list }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.list.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.list.clone()
    }
}
