use std::io::{self, stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let result = run_game();

    disable_raw_mode()?;

    result
}

fn run_game() -> io::Result<()> {
    let mut output = stdout();

    writeln!(output, "Terminal input test")?;
    writeln!(output, "Press arrow keys or H J K L.")?;
    writeln!(output, "Press Q or Esc to quit.")?;
    writeln!(output)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    writeln!(output, "Quitting.")?;
                    break;
                }

                KeyCode::Up | KeyCode::Char('k') => {
                    writeln!(output, "You pressed: up")?;
                }

                KeyCode::Down | KeyCode::Char('j') => {
                    writeln!(output, "You pressed: down")?;
                }

                KeyCode::Left | KeyCode::Char('h') => {
                    writeln!(output, "You pressed: left")?;
                }

                KeyCode::Right | KeyCode::Char('l') => {
                    writeln!(output, "You pressed: right")?;
                }

                _ => {}
            }

            output.flush()?;
        }
    }

    Ok(())
}