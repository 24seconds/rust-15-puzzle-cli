mod helper;
use helper::{
    draw_board, draw_header, handle_game_state, handle_move_operation, move_tile,
    update_elapsed_time, Event, Events, GameData, GameState, Operation,
};

use std::{error::Error, io, time::Instant};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
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
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();
    let mut rng = rand::thread_rng();

    let mut game_data = GameData::new(&mut rng);

    loop {
        terminal.draw(|mut f| {
            let layout_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0), // main render
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
                draw_header(
                    &mut f,
                    &layout_chunks[0].inner(&Margin {
                        horizontal: 10,
                        vertical: 0,
                    }),
                    &game_data.game_state,
                )
                .unwrap();
            }

            {
                let time = match game_data.game_state {
                    GameState::INIT => {
                        game_data.start_time = Instant::now();

                        0
                    }
                    GameState::PLAYING => {
                        game_data.base_time + game_data.start_time.elapsed().as_secs()
                    }
                    GameState::PAUSED => {
                        game_data.start_time = Instant::now();

                        game_data.base_time
                    }
                    GameState::DONE => game_data.base_time,
                };

                let title_string = format!(" Time: {}s  Moves: {}", time, &game_data.move_count);
                let title_string = title_string.as_str();

                let block = Block::default()
                    .borders(Borders::NONE)
                    .title(title_string)
                    .title_style(Style::default().modifier(Modifier::BOLD));
                f.render_widget(block, chunks[1]);

                draw_board(
                    &game_data.arr_state,
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
                    let next_arr_state = move_tile(&game_data.arr_state, Operation::UP)?;
                    handle_move_operation(&mut game_data, next_arr_state, 'w');
                }
                Key::Char('a') | Key::Left => {
                    let next_arr_state = move_tile(&game_data.arr_state, Operation::LEFT)?;
                    handle_move_operation(&mut game_data, next_arr_state, 'a');
                }
                Key::Char('s') | Key::Down => {
                    let next_arr_state = move_tile(&game_data.arr_state, Operation::DOWN)?;
                    handle_move_operation(&mut game_data, next_arr_state, 's');
                }
                Key::Char('d') | Key::Right => {
                    let next_arr_state = move_tile(&game_data.arr_state, Operation::RIGHT)?;
                    handle_move_operation(&mut game_data, next_arr_state, 'd');
                }
                Key::Char('p') => {
                    let next_game_state = handle_game_state(&game_data, 'p');

                    game_data.base_time = update_elapsed_time(&game_data, &next_game_state);
                    game_data.game_state = next_game_state;
                }
                Key::Char('r') => {
                    game_data = GameData::new(&mut rng);
                    let next_game_state = handle_game_state(&game_data, 'r');
                    game_data.game_state = next_game_state;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
