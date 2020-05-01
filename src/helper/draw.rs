use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Text},
    Frame,
};

pub fn draw_board<B>(
    arr: &[u16; 16],
    frame: &mut Frame<B>,
    area: &Rect,
    length: u16,
) -> Result<(), Box<dyn Error>>
where
    B: Backend,
{
    let board = [
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (0, 2),
        (1, 2),
        (2, 2),
        (3, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
    ];

    board.iter().zip(arr.iter()).enumerate().for_each(|x| {
        let (index, (multiplier, number)) = x;
        let width = length + 3;
        let height = length;
        let area = Rect::new(
            area.x + width * multiplier.0,
            area.y + length * multiplier.1,
            width,
            height,
        );

        let style_selected = Style::default().fg(if index as u16 + 1 == *number && *number != 0 {
            // orange
            Color::Rgb(232, 138, 69)
        } else {
            Color::White
        });

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(style_selected);

        let number_string = if *number == 0 {
            String::from("")
        } else {
            format!("\n{}", number)
        };

        let text = [Text::styled(number_string, style_selected.modifier(Modifier::BOLD))];
        let paragraph = Paragraph::new(text.iter())
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
        frame.render_widget(block, area);
    });

    Ok(())
}

pub fn draw_guide<B>(frame: &mut Frame<B>, area: &Rect) -> Result<(), Box<dyn Error>>
where
    B: Backend,
{
    let guide = r#"    

Commands 
    Move: ↑,↓,←,→ or w,s,a,d
    Quit : q
    New game : r
    Pause : p
    Solve : g
    "#;

    let block = Block::default()
        .borders(Borders::NONE)
        .title("rust-15-puzzle : v0.1.0")
        .title_style(Style::default().modifier(Modifier::BOLD));
    let text = [Text::styled(guide, Style::default().fg(Color::LightBlue))];
    let paragraph = Paragraph::new(text.iter())
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, *area);

    Ok(())
}
