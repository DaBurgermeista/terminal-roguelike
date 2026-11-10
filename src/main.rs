use std::io::{self, stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

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
    enable_raw_mode()?;

    let result = run_game();

    disable_raw_mode()?;

    result
}

fn run_game() -> io::Result<()> {
    let mut player_x: i32 = 5;
    let mut player_y: i32 = 5;

    println!("Terminal Roguelike");
    println!("Move: arrow keys or H J K L");
    println!("Quit: Q or Esc");

    loop {
        draw_dungeon(player_x, player_y)?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Char('k') => {
                    try_move(&mut player_x, &mut player_y, 0, -1);
                }

                KeyCode::Down | KeyCode::Char('j') => {
                    try_move(&mut player_x, &mut player_y, 0, 1);
                }

                KeyCode::Left | KeyCode::Char('h') => {
                    try_move(&mut player_x, &mut player_y, -1, 0);
                }

                KeyCode::Right | KeyCode::Char('l') => {
                    try_move(&mut player_x, &mut player_y, 1, 0);
                }

                _ => continue,
            }
        }
    }

    println!("\nThanks for playing.");
    Ok(())
}

fn try_move(player_x: &mut i32, player_y: &mut i32, dx: i32, dy: i32) {
    let next_x = *player_x + dx;
    let next_y = *player_y + dy;

    if is_walkable(next_x, next_y) {
        *player_x = next_x;
        *player_y = next_y;
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

fn draw_dungeon(player_x: i32, player_y: i32) -> io::Result<()> {
    let mut output = stdout();

    writeln!(output)?;

    for (y, row) in DUNGEON.iter().enumerate() {
        let mut rendered_row = String::with_capacity(row.len());

        for (x, tile) in row.chars().enumerate() {
            if x as i32 == player_x && y as i32 == player_y {
                rendered_row.push('@');
            } else {
                rendered_row.push(tile);
            }
        }

        writeln!(output, "{rendered_row}")?;
    }

    output.flush()?;
    Ok(())
}