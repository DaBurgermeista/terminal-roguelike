// src/player.rs
use crate::room::Position;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,      // Determines attack hit chance
    pub strength: i32,    // Determines damage (Max hit ceiling)
    pub defense: i32,     // Determines chance for incoming attacks to miss and damage mitigation
    pub position: Position,
}

impl Player {

    pub fn new(name: String, position: Position) -> Self {
        Player {
            name,
            health: 10,
            max_health: 10,
            attack: 3,
            strength: 3,
            defense: 3,
            position,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

}
