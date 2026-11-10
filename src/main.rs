use inquire::{error::InquireError, Select};

mod player;
mod room;

use player::Player;
use room::Room;

fn main() {
    run_main_menu();
}

fn start_new_game() {
    let player = Player::new(String::from("Adventurer"));
    let starting_room = Room::starting_room();

    println!("\nWelcome, {}.", player.name);
    println!("\n{}", starting_room.name);
    println!("{}", "-".repeat(starting_room.name.len()));
    println!("{}", starting_room.description);
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
