use std::io::{self, Stdout, stdout};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::thermal::ThermalPressure;

// 5 levels packed into 3 terminal rows using lower half blocks
const CHART_ROWS: u16 = 3;

fn pressure_color(p: ThermalPressure) -> Color {
    match p {
        ThermalPressure::Nominal => Color::White,
        ThermalPressure::Moderate => Color::Yellow,
        ThermalPressure::Heavy => Color::Red,
        ThermalPressure::Trapping => Color::Red,
        ThermalPressure::Sleeping => Color::Magenta,
        ThermalPressure::Unknown(_) => Color::White,
    }
}

pub struct BarDisplay {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    history: Vec<ThermalPressure>,
}

impl BarDisplay {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::with_options(
            backend,
            ratatui::TerminalOptions {
                viewport: ratatui::Viewport::Inline(CHART_ROWS + 1),
            },
        )?;
        Ok(Self {
            terminal,
            history: Vec::new(),
        })
    }

    pub fn push(&mut self, pressure: ThermalPressure) {
        let max_cols = self.terminal.size().map(|r| r.width as usize).unwrap_or(80);
        if self.history.len() >= max_cols {
            self.history.remove(0);
        }
        self.history.push(pressure);
    }

    pub fn draw(&mut self, countdown: &str) -> io::Result<()> {
        let Self { terminal, history } = self;
        let countdown = countdown.to_string();
        terminal.draw(|frame| {
            let area = frame.area();
            let buf = frame.buffer_mut();

            for (col, p) in history.iter().enumerate() {
                if col as u16 >= area.width {
                    break;
                }
                let height = p.level() + 1; // 1–5
                let style = Style::default().fg(pressure_color(*p));

                for row in 0..CHART_ROWS {
                    let bottom_unit = (CHART_ROWS - 1 - row) * 2 + 1;
                    let top_unit = bottom_unit + 1;
                    let symbol = if height >= top_unit as u64 {
                        "█"
                    } else if height >= bottom_unit as u64 {
                        "▄"
                    } else {
                        " "
                    };
                    buf.set_string(area.x + col as u16, area.y + row, symbol, style);
                }
            }

            let countdown_area = Rect::new(area.x, area.y + CHART_ROWS, area.width, 1);
            Paragraph::new(Line::from(Span::styled(
                countdown,
                Style::default().add_modifier(Modifier::DIM),
            )))
            .render(countdown_area, buf);
        })?;
        Ok(())
    }

    pub fn print_status(&mut self, pressure: ThermalPressure) -> io::Result<()> {
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let label_style = match pressure {
            ThermalPressure::Nominal => Style::default().fg(Color::Green).bold(),
            ThermalPressure::Moderate => Style::default().fg(Color::Yellow).bold(),
            ThermalPressure::Heavy => Style::default().fg(Color::Red).bold(),
            ThermalPressure::Trapping => Style::default().fg(Color::Red).bold().underlined(),
            ThermalPressure::Sleeping => Style::default().fg(Color::Magenta).bold().underlined(),
            ThermalPressure::Unknown(_) => Style::default().dim(),
        };
        let dim = Style::default().dim();
        let line = Line::from(vec![
            Span::styled(now, dim),
            Span::raw(" "),
            Span::styled(pressure.to_string(), label_style),
            Span::raw(" "),
            Span::styled(format!("({})", pressure.description()), dim),
        ]);
        self.terminal.insert_before(1, |buf| {
            let area = buf.area;
            line.render(area, buf);
        })?;
        Ok(())
    }
}
