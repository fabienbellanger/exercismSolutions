// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health > 0 {
            true => None,
            false => {
                let mana = match self.level >= 10 {
                    true => Some(100),
                    false => None,
                };

                Some(Self {
                    health: 100,
                    level: self.level,
                    mana,
                })
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = match self.health > mana_cost {
                    true => self.health - mana_cost,
                    false => 0,
                };

                0
            }
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);

                mana_cost * 2
            }
            _ => 0,
        }
    }
}
