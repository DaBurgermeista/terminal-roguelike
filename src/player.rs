// src/player.rs

#[derive(Debug)]
pub struct Player {
    pub name: String,
    //pub class: String,  // Might make this it's own struct later
}

impl Player {

    pub fn new(name: String) -> Self {
        Player {
            name,
        }
    }

}