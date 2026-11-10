mod player;
mod room;
mod world;

use inquire::{error::InquireError, Select};

use world::{World, STARTING_POSITION};
use player::Player;

fn main() {
    run_main_menu();
}

fn start_new_game() {
    let world = World::new();

    let mut player = Player::new(
        String::from("Adventurer"),
        STARTING_POSITION,
    );

    let current_room = world
        .room_at(player.position)
        .expect("Starting position must contain a room");

    let exits = world.available_exits(player.position);

    println!("\nWelcome, {}.", player.name);
    println!("\n{}", current_room.name);
    println!("{}", "-".repeat(current_room.name.len()));
    println!("{}", current_room.description);
    println!("\nExits: {}", exits.join(", "));
    println!("\nYour adventure begins here.");

    let ans = Select::new("Go where:", exits).prompt();

    match ans {
        Ok("north") => {
            println!("You travel north.");
        }
        Ok("south") => {
            println!("You travel south.");
        }
        Ok("east") => {
            println!("You travel east.");
        }
        Ok("west") => {
            println!("You travel west.");
        }
        Ok(other) => {
            println!("Unexpected direction: {other}");
        }
        Err(InquireError::OperationCanceled) => {
            println!("Movement cancled");
        }
        Err(error) => {
            println!("There was an error: {error}");
        }
    }
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
