mod helper;
use helper::{draw_board, move_cell, Operation, Event, Events};

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    let mut arr_state = helper::shuffle_arr(&mut rand::thread_rng())?;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10), // guide to user
                        Constraint::Percentage(80), // main render
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            {
                // let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
                draw_board(&arr_state, &mut f, &chunks[1], 5).unwrap();
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Char('w') | Key::Up => {
                    arr_state = move_cell(&arr_state, Operation::UP)?;
                }
                Key::Char('a') | Key::Left => {
                    arr_state = move_cell(&arr_state, Operation::LEFT)?;
                }
                Key::Char('s') | Key::Down => {
                    arr_state = move_cell(&arr_state, Operation::DOWN)?;
                }
                Key::Char('d') | Key::Right => {
                    arr_state = move_cell(&arr_state, Operation::RIGHT)?;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
