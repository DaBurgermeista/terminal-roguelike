use std::io::{self, Write};

struct Room {
    name: &'static str,
    description: &'static str,
}

fn main() -> io::Result<()> {
    let entrance_hall = Room {
        name: "Entrance Hall",
        description: "You stand in a cold stone hall beneath the ruins of an "
            "ancient fortress. Water drips steadily from the ceiling. A heavy "
            "wooden door stands to the north.",
    };

    let mut message = "Type LOOK to examine the room, or Q to quit.";

    println!("Terminal Roguelike");
    println!("A text-adventure dungeon crawler.");

    loop {
        display_room(&entrance_hall, message);

        print!("\nCommand: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim().to_lowercase();

        message = match command.as_str() {
            "q" | "quit" => {
                println!("\nThanks for playing.");
                break;
            }
            "look" | "l" => "You look around the Entrance Hall.",
            "" => "You hesitate.",
            _ => "Unknown command. Type LOOK or Q.",
        };
    }

    Ok(())
}

fn display_room(room: &Room, message: &str) {
    println!();
    println!("{}", room.name);
    println!("{}", "-".repeat(room.name.len()));
    println!("{}", room.description);
    println!();
    println!("Message: {message}");
}