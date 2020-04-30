use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
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

    board.iter().zip(arr.iter()).for_each(|x| {
        let (multiplier, number) = x;
        let width = length + 3;
        let height = length;
        let area = Rect::new(
            area.x + width * multiplier.0,
            area.y + length * multiplier.1,
            width,
            height,
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let number_string = if *number == 0 {
            String::from("")
        } else {
            format!("\n{}", number)
        };

        let text = [Text::styled(
            number_string,
            Style::default().fg(Color::White),
        )];
        let paragraph = Paragraph::new(text.iter())
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
        frame.render_widget(block, area);
    });

    Ok(())
}
