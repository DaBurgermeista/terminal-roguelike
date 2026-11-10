mod player;
mod room;

use inquire::{error::InquireError, Select};

use player::Player;
use room::{Room, Position};

fn main() {
    run_main_menu();
}

fn start_new_game() {
    let rooms = vec![
        Room::starting_room(),
    ];
    let player = Player::new(String::from("Adventurer"), Position {x: 0, y: 0});
    let current_room = &rooms[0];

    println!("\nWelcome, {}.", player.name);
    println!("\n{}", current_room.name);
    println!("{}", "-".repeat(current_room.name.len()));
    println!("{}", current_room.description);
    println!("\nYour adventure begins here.");
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
