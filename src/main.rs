mod helper;
use helper::{draw_board, move_tile, Event, Events, Operation};

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
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
            let layout_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(90), // main render
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Length(40),
                        Constraint::Min(0),
                    ]
                    .as_ref(),
                )
                .split(layout_chunks[1]);

            let footer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(23), Constraint::Min(0)].as_ref())
                .split(chunks[1]);

            {
                let block = Block::default()
                    .borders(Borders::NONE)
                    .title(" Time: 23s  Moves: 15")
                    .title_style(Style::default().modifier(Modifier::BOLD));
                f.render_widget(block, chunks[1]);

                draw_board(
                    &arr_state,
                    &mut f,
                    &chunks[1].inner(&Margin {
                        horizontal: 1,
                        vertical: 2,
                    }),
                    5,
                )
                .unwrap();
            }

            {
                helper::draw_guide(&mut f, &chunks[2]).unwrap();
            }

            {
                let footer = "🍺 Github: 24seconds/rust-15-puzzle-cli";
                let block = Block::default()
                    .borders(Borders::NONE)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(footer);
                f.render_widget(block, footer_chunks[1]);
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Char('w') | Key::Up => {
                    arr_state = move_tile(&arr_state, Operation::UP)?;
                }
                Key::Char('a') | Key::Left => {
                    arr_state = move_tile(&arr_state, Operation::LEFT)?;
                }
                Key::Char('s') | Key::Down => {
                    arr_state = move_tile(&arr_state, Operation::DOWN)?;
                }
                Key::Char('d') | Key::Right => {
                    arr_state = move_tile(&arr_state, Operation::RIGHT)?;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
