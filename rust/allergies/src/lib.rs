#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Allergies {
    list: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,         //   1 = 2^0
    Peanuts = 1 << 1,      //   2 = 2^1
    Shellfish = 1 << 2,    //   4 = 2^2
    Strawberries = 1 << 3, //   8 = 2^3
    Tomatoes = 1 << 4,     //  16 = 2^4
    Chocolate = 1 << 5,    //  32 = 2^5
    Pollen = 1 << 6,       //  64 = 2^6
    Cats = 1 << 7,         // 128 = 2^7
}

const ALLERGENS: &[Allergen; 8] = &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        let n = ALLERGENS.len();
        let mut score = score % 256; // Max = 255 => All allergens
        let mut list = Vec::with_capacity(n);

        for i in 0..n {
            if score == 0 {
                break;
            }

            let j: usize = n - i - 1;
            let value = 1u32 << (n - i - 1);

            if value > score {
                continue;
            }

            list.push((&ALLERGENS[j]).clone());

            score -= value;
        }

        Self { list }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.list.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.list.clone()
    }
}
