
use lume_core::core::{Matrix, Renderable};
use ratatui::{prelude::*, widgets::*};


use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

pub fn draw_cli(matrix: &mut Matrix,pattern: &mut dyn Renderable,ms:u64) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;


    loop {
        terminal.draw(|frame| ui(frame, &matrix))?;

        if event::poll(std::time::Duration::from_millis(ms))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') { break; }
            }
        }

        pattern.render(matrix);
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}


pub fn format_matrix_leds(matrix: &Matrix) -> Vec<Line<'_>> {

    let mut out:Vec<Line> = Vec::new();
    for bit_pos in (0..16).rev() {
        let mut spans = Vec::new();
        for row_val in &matrix.rows {
            let is_on = (row_val >> bit_pos) & 1 == 1;

            let (symbol, color) = if is_on {
                (" ⬤ ", Color::Red)
            } else {
                (" ⬤ ", Color::Indexed(235))
            };

            spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
        out.push(Line::from(spans));
    }
    out
}
pub fn ui(frame: &mut Frame, matrix: &Matrix) {
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),            
            Constraint::Length(16),
            Constraint::Fill(1),             
        ])
        .split(area);

    let center_area = chunks[1];

    let lines = format_matrix_leds(matrix);
    let matrix_width = if let Some(first_line) = lines.get(0) {
        (first_line.width() as u16 ).min(area.width)
    } else {
        2
    };

    let final_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(matrix_width+5),
            Constraint::Fill(1),
        ])
        .split(center_area)[1];


    let paragraph = Paragraph::new(format_matrix_leds(matrix))
        .block(Block::default().title(" LED Matrix TUI ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, final_area);
}
