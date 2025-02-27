mod driver;
mod ratatui_ui;
mod scraper;

use crate::ratatui_ui::InputMode;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{future, FutureExt};
use rand::distributions::Distribution;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Terminal,
};
use std::{fs};
use std::future::IntoFuture;
use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, io};
use thirtyfour::prelude::*;
use tokio;
use tui_input::backend::crossterm::EventHandler;

async fn watchdog_logic(config_path: &str) {
    let config_path_fmt = fs::read_to_string(Path::new(&config_path)).unwrap();

    let chromedriver_config: driver::ChromedriverConfig = toml::from_str(config_path_fmt.as_str()).unwrap();
    driver::start_chromedriver(chromedriver_config.chromedriver_path).await;

    let configs: scraper::ScraperConfigVec = toml::from_str(config_path_fmt.as_str()).unwrap();
    let mut scraper_structs = vec![];
    for config in configs.scraper {
        scraper::from_config(&mut scraper_structs, config).await;
    }

    let futures: Vec<_> = scraper_structs
        .iter_mut()
        .map(|scraper| scraper.run())
        .collect();
    let results = future::join_all(futures).await;
}

// Debug logic without UI
// #[tokio::main]
// async fn main() {
//     watchdog_logic("/path/to/config.toml").await;
// }

#[tokio::main]
async fn main() {
    // Setup terminal
    enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen); //EnableMouseCapture
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create app state
    let mut app = ratatui_ui::App::new();

    let logo = r"   __ __             _            _      __     __      __     __
  / // ___ __ _____ (____ ___ _  | | /| / ___ _/ /_____/ / ___/ ___ ___ _
 / _  / _ / // (_-</ / _ / _ `/  | |/ |/ / _ `/ __/ __/ _ / _  / _ / _ `/
/_//_/\___\_,_/___/_/_//_\_, /   |__/|__/\_,_/\__/\__/_//_\_,_/\___\_, /
                        /___/                                     /___/  ";

    // Main loop
    loop {
        // Update sparkline if loading
        if app.loading {
            app.update_sparkline();
        }

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(8), // ASCII logo - increased height
                            Constraint::Length(3), // File path input
                            Constraint::Length(3), // Buttons
                            Constraint::Length(6), // Sparkline - increased height
                            Constraint::Min(1),    // Output
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // ASCII logo
                let logo_widget = Paragraph::new(logo)
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::ALL).title(""));
                f.render_widget(logo_widget, chunks[0]);

                // File path input
                let file_path_style = match app.input_mode {
                    InputMode::FilePathInput => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                };

                let file_path_input = Paragraph::new(app.file_path_input.value())
                    .style(file_path_style)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" config.toml file path "),
                    );
                f.render_widget(file_path_input, chunks[1]);

                // Create horizontal layout for buttons
                let button_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[2]);

                // Submit button
                let submit_style = match app.input_mode {
                    InputMode::RunButton => Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                    _ => Style::default(),
                };

                let submit_button = Paragraph::new("[ Run ]")
                    .style(submit_style)
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(submit_button, button_chunks[0]);

                // Stop button
                let stop_style = match app.input_mode {
                    InputMode::StopButton => Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                    _ => Style::default(),
                };

                let stop_button = Paragraph::new("[ Stop ]")
                    .style(stop_style)
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(stop_button, button_chunks[1]);

                // Sparkline
                let sparkline_data: Vec<u64> = app.sparkline_data.iter().copied().collect();
                let sparkline = Sparkline::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(if app.loading {
                                "Scraping (Processing...)"
                            } else {
                                "Scraping (Stopped)"
                            }),
                    )
                    .data(&sparkline_data)
                    .style(Style::default().fg(if app.loading {
                        Color::Green
                    } else {
                        Color::Gray
                    }));
                f.render_widget(sparkline, chunks[3]);

                // Output
                if app.show_output {
                    let output = Paragraph::new(app.output.clone())
                        .block(Block::default().borders(Borders::ALL).title("Result"));
                    f.render_widget(output, chunks[4]);
                }
            })
            .unwrap();

        // Handle events with a small timeout to allow for animation
        if crossterm::event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Tab => {
                        app.next_input();
                    }
                    KeyCode::Enter => match app.input_mode {
                        InputMode::RunButton => app.run_watchdogs(),
                        InputMode::StopButton => app.stop_watchdogs(),
                        _ => {}
                    },
                    _ => match app.input_mode {
                        InputMode::FilePathInput => {
                            app.file_path_input.handle_event(&Event::Key(key));
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    terminal.show_cursor();
}
