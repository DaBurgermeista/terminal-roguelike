// src/player.rs

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,      // Determines attack hit chance
    pub strength: i32,    // Determines damage (Max hit ceiling)
    pub defense: i32,     // Determines chance for incoming attacks to miss and damage mitigation
}

impl Player {

    pub fn new(name: String) -> Self {
        Player {
            name,
            health: 10,
            max_health: 10,
            attack: 3,
            strength: 3,
            defense: 3,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;

        if self.health < 0 {
            self.health = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;

        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn new_player_uses_provided_name() {
        let player = Player::new(String::from("Adventurer"));

        assert_eq!(player.name, "Adventurer")
    }

    #[test]
    fn new_player_has_starting_stats() {
        let player = Player::new(String::from("Adventurer"));

        assert_eq!(player.health, 10);
        assert_eq!(player.max_health, 10);
        assert_eq!(player.attack, 3);
        assert_eq!(player.strength, 3);
        assert_eq!(player.defense, 3);
    }

    #[test]
    fn test_damage() {
        let mut player = Player::new(String::from("Adventurer"));

        player.take_damage(5);
        assert_eq!(player.health, 5);

        player.heal(5);
        assert_eq!(player.health, 10);

        player.heal(5);
        assert_eq!(player.health, 10);

        player.take_damage(50);
        assert_eq!(player.health, 0);
}
}