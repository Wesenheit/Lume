
use lume_core::core::{Matrix, Renderable};
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pallete {
    Light,
    Dark,
}

impl Pallete {
    pub fn on(&self) -> Color{
        match self {
            Pallete::Dark => Color::Red,
            Pallete::Light => Color::Green
        }
    }
    pub fn off(&self) -> Color{
        match self {
            Pallete::Dark => Color::Indexed(235),
            Pallete::Light => Color::Reset,
        }
    }
}

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

pub fn draw_cli(matrix: &mut Matrix,pattern: &mut dyn Renderable,ms:u64,theme: Pallete) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;


    loop {
        terminal.draw(|frame| ui(frame, &matrix,theme))?;

        if event::poll(std::time::Duration::from_millis(ms))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') { break; }
            }
        }

        matrix.update(pattern);
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}



pub fn format_matrix_leds(matrix: &Matrix,theme: Pallete) -> Vec<Line<'_>> {
    let height_of_led = if matrix.reduce {8} else {16};
    let mut out:Vec<Line> = Vec::new();
    for bit_pos in (0..height_of_led).rev() {
        let mut spans = Vec::new();
        spans.push(Span::styled(" ",Style::default()));
        let size = matrix.rows.len();        
        for i in 1..size {
            let is_on:bool = if matrix.reduce {
                (matrix.rows_u8[i] >> bit_pos) & 1 == 1
            } else {
                (matrix.rows[i] >> bit_pos) & 1 == 1
            };
            let (symbol, color) = if is_on {
                ("⬤ ", theme.on())
            } else {
                ("⬤ ", theme.off())
            };

            spans.push(Span::styled(symbol, Style::default().fg(color).bg(Color::Reset)));
            spans.push(Span::styled(" ",Style::default()));
        }
        out.push(Line::from(spans));
    }
    out
}
pub fn ui(frame: &mut Frame, matrix: &Matrix,theme: Pallete) {
    let height_of_led = if matrix.reduce {8} else {16};
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),            
            Constraint::Length(height_of_led+2),
            Constraint::Fill(1),             
        ])
        .split(area);

    let center_area = chunks[1];

    let lines = format_matrix_leds(matrix,theme);
    let matrix_width = if let Some(first_line) = lines.get(0) {
        (first_line.width() as u16 ).min(area.width)
    } else {
        2
    };

    let final_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(matrix_width+2),
            Constraint::Fill(1),
        ])
        .split(center_area)[1];


    let paragraph = Paragraph::new(format_matrix_leds(matrix,theme))
        .block(Block::default().title(" LED Matrix TUI ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, final_area);
}
