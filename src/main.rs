use std::io::{self, Write};

const DUNGEON: &[&str] = &[
    "##############################",
    "#............................#",
    "#............................#",
    "#........##########..........#",
    "#........#........#..........#",
    "#........#........#..........#",
    "#........#........#..........#",
    "#........##########..........#",
    "#............................#",
    "#............................#",
    "#............................#",
    "##############################",
];

fn main() -> io::Result<()> {
    let mut player_x: i32 = 5;
    let mut player_y: i32 = 5;
    let mut message = String::from("You enter the dungeon.");

    println!("Terminal Roguelike");
    println!("Move with W A S D, then press Enter.");
    println!("Enter Q to quit.");

    loop {
        draw_dungeon(player_x, player_y, &message);

        print!("\nMove (WASD, Q to quit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim().to_lowercase();

        message = match command.as_str() {
            "q" => {
                println!("Thanks for playing.");
                break;
            }
            "w" => try_move(&mut player_x, &mut player_y, 0, -1),
            "s" => try_move(&mut player_x, &mut player_y, 0, 1),
            "a" => try_move(&mut player_x, &mut player_y, -1, 0),
            "d" => try_move(&mut player_x, &mut player_y, 1, 0),
            "" => String::from("You wait."),
            _ => String::from("Unknown command. Use W, A, S, D, or Q."),
        };
    }

    Ok(())
}

fn try_move(player_x: &mut i32, player_y: &mut i32, dx: i32, dy: i32) -> String {
    let next_x = *player_x + dx;
    let next_y = *player_y + dy;

    if is_walkable(next_x, next_y) {
        *player_x = next_x;
        *player_y = next_y;

        String::from("You move.")
    } else {
        String::from("You bump into a wall.")
    }
}

fn is_walkable(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if x >= DUNGEON[0].len() as i32 || y >= DUNGEON.len() as i32 {
        return false;
    }

    DUNGEON[y as usize].as_bytes()[x as usize] != b'#'
}

fn draw_dungeon(player_x: i32, player_y: i32, message: &str) {
    println!();

    for (y, row) in DUNGEON.iter().enumerate() {
        let mut rendered_row = String::with_capacity(row.len());

        for (x, tile) in row.chars().enumerate() {
            if x as i32 == player_x && y as i32 == player_y {
                rendered_row.push('@');
            } else {
                rendered_row.push(tile);
            }
        }

        println!("{rendered_row}");
    }

    println!();
    println!("Message: {message}");
}