// src/room.rs

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
}

impl Room {
    pub fn new(name: String, description: String) -> Self {
        Room { name, description }
    }

    pub fn starting_room() -> Self {
        Room::new(
            String::from("The Old Road"),
            String::from(
                "You wake on a rain-soaked road at the edge of a dark forest. \
                A ruined watchtower rises to the north. The road continues east.",
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Room;

    #[test]
    fn starting_room_has_a_name_and_description() {
        let room = Room::starting_room();

        assert!(!room.name.is_empty());
        assert!(!room.description.is_empty());
    }
}