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

impl Allergies {
    pub fn new(score: u32) -> Self {
        // Max = 255 => All allergens
        let score = score % 256;

        let mut list = Vec::with_capacity(8);
        let mut remaining = score;

        for i in 0..8 {
            if remaining == 0 {
                break;
            }

            let value = 1u32 << (8 - i - 1);

            if value > remaining {
                continue;
            }

            let allergen = match i {
                0 => Allergen::Cats,
                1 => Allergen::Pollen,
                2 => Allergen::Chocolate,
                3 => Allergen::Tomatoes,
                4 => Allergen::Strawberries,
                5 => Allergen::Shellfish,
                6 => Allergen::Peanuts,
                7 => Allergen::Eggs,
                _ => panic!("invalid number"),
            };

            list.push(allergen.clone());

            remaining -= value;
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
