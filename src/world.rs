// src/world.rs
use std::collections::HashMap;

use crate::room::{Position, Room};

pub const STARTING_POSITION: Position = Position { x: 0, y: 0 };

pub struct World {
    rooms: HashMap<Position, Room>,
}

impl World {
    pub fn new() -> Self {
        let mut rooms = HashMap::new();

        rooms.insert(STARTING_POSITION, Room::starting_room());

        rooms.insert(
            Position { x: 0, y: -1 },
            Room::new(
                String::from("Ruined Watchtower"),
                String::from(
                    "A crumbling watchtower overlooks the forest. Wind whistles \
                     through gaps in the broken stone walls.",
                ),
            ),
        );

        rooms.insert(
            Position { x: 1, y: 0 },
            Room::new(
                String::from("Forest Road"),
                String::from(
                    "The old road continues east beneath twisted trees. Deep wagon \
                     ruts disappear into the mist.",
                ),
            ),
        );

        World { rooms }
    }

    pub fn room_at(&self, position: Position) -> Option<&Room> {
        self.rooms.get(&position)
    }

    pub fn available_exits(&self, position: Position) -> Vec<&'static str> {
        let possible_exits = [
            (
                "north",
                Position {
                    x: position.x,
                    y: position.y - 1,
                },
            ),
            (
                "south",
                Position {
                    x: position.x,
                    y: position.y + 1,
                },
            ),
            (
                "east",
                Position {
                    x: position.x + 1,
                    y: position.y,
                },
            ),
            (
                "west",
                Position {
                    x: position.x - 1,
                    y: position.y,
                },
            ),
        ];

        possible_exits
            .iter()
            .filter_map(|(name, destination)| self.rooms.contains_key(destination).then_some(*name))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{World, STARTING_POSITION};

    #[test]
    fn starting_position_contains_a_room() {
        let world = World::new();

        assert!(world.room_at(STARTING_POSITION).is_some());
    }

    #[test]
    fn starting_room_has_north_and_east_exits() {
        let world = World::new();

        let exits = world.available_exits(STARTING_POSITION);

        assert_eq!(exits, vec!["north", "east"]);
    }
}