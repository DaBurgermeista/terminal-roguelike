mod player;
mod room;

use inquire::{error::InquireError, Select};
use std::collections::HashMap;

use player::Player;
use room::{Room, Position};

fn main() {
    run_main_menu();
}

fn start_new_game() {
    let rooms = build_starting_world();

    let player = Player::new(
        String::from("Adventurer"),
        Position { x: 0, y: 0 },
    );

    let current_room = rooms
        .get(&player.position)
        .expect("Starting position must contain a room");

    let exits = available_exits(&rooms, player.position);

    println!("\nWelcome, {}.", player.name);
    println!("\n{}", current_room.name);
    println!("{}", "-".repeat(current_room.name.len()));
    println!("{}", current_room.description);
    println!("\nExits: {}", exits.join(", "));
    println!("\nYour adventure begins here.");
}

fn build_starting_world() -> HashMap<Position, Room> {
    let mut rooms = HashMap::new();

    rooms.insert(Position { x: 0, y: 0 }, Room::starting_room());

    rooms.insert(
        Position { x: 0, y: -1 },
        Room::new(
            String::from("Ruined Watchtower"),
            String::from(
                "A crumbling watchtower overlooks the forest. Wind whistles ",
                    "through gaps in the broken stone walls.",
            ),
        ),
    );

    rooms.insert(
        Position { x: 1, y: 0 },
        Room::new(
            String::from("Forest Road"),
            String::from(
                "The old road continues east beneath twisted trees. Deep wagon ",
                    "ruts disappear into the mist.",
            ),
        ),
    );

    rooms
}

fn available_exits(
    rooms: &HashMap<Position, Room>,
    position: Position,
) -> Vec<&'static str> {
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
        .filter_map(|(name, destination)| rooms.contains_key(destination).then_some(*name))
        .collect()
}

fn run_main_menu() {

    let options: Vec<&str> = vec![
        "New Game",
        "Quit Game",
    ];

    let ans: Result<&str, InquireError> = Select::new("Choose a menu option:", options).prompt();

    match ans {
        Ok("New Game") => {
            start_new_game();
        }
        Ok("Quit Game") => println!("Goodbye."),
        Ok(other) => println!("Unexpected menu choice: {other}"),
        Err(InquireError::OperationCanceled) => {
            println!("Menu canceled.");
        }
        Err(error) => {
            println!("There was an error: {error}");
        }
    }
}
