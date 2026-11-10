use inquire::{error::InquireError, Select};

mod player;
use player::Player;

fn main() {
    run_main_menu();
}

fn run_main_menu() {
    let player = Player::new(String::from("Adventurer"));

    println!("Welcome, {}.", player.name);

    let options: Vec<&str> = vec![
        "New Game",
        "Quit Game",
    ];

    let ans: Result<&str, InquireError> = Select::new("Choose a menu option:", options).prompt();

    match ans {
        Ok("New Game") => println!("Starting a new game..."),
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
