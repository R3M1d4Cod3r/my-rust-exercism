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
        match self.health{
            0 => { match self.level { 
                0..=9  => Some(Player{health : 100 , mana : None , level : self.level}),
                10..    => Some(Player{health : 100 , mana : Some(100) , level : self.level}),
            }},
            _ => None, 
        }

    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana{
            None    => { self.health = if self.health > mana_cost {self.health-mana_cost} else {0}; 0 },
            Some(x) => { if x < mana_cost {  0 } else {self.mana=Some(x-mana_cost); mana_cost*2} },
        }
    }
}
