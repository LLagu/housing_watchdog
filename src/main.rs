mod driver;
mod scraper;
use futures::{future, FutureExt};
use std::future::IntoFuture;
use std::io::prelude::*;
use std::path::Path;
use std::{fs};
use thirtyfour::prelude::*;
use tokio;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Terminal,
};
use std::{collections::VecDeque, error::Error, io, time::Instant};
use tokio::sync::oneshot;
use tui_input::{backend::crossterm::EventHandler, Input};

async fn watchdog_logic(config_path: &str) {
    driver::start_chromedriver().await;
    let config_path_fmt = fs::read_to_string(Path::new(&config_path)).unwrap();
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

enum InputMode {
    NtfyTopic,
    FilePathInput,
    RunButton,
    StopButton,
}

struct App {
    ntfy_topic_input: Input,
    file_path_input: Input,
    input_mode: InputMode,
    output: String,
    show_output: bool,
    loading: bool,
    sparkline_data: VecDeque<u64>,
    rng: ThreadRng,
    loading_start_time: Option<Instant>,
    cancel_sender: Option<oneshot::Sender<()>>,
}

impl App {
    fn new() -> App {
        // Initialize with some random data to fill the sparkline
        let mut initial_data = VecDeque::with_capacity(100);
        let mut rng = rand::thread_rng();
        // let between = Uniform::from(0..100);

        for _ in 0..100 {
            // initial_data.push_back(between.sample(&mut rng));
            initial_data.push_back(0);
        }

        App {
            ntfy_topic_input: Input::default(),
            file_path_input: Input::default(),
            input_mode: InputMode::NtfyTopic,
            output: String::new(),
            show_output: false,
            loading: false,
            sparkline_data: initial_data,
            rng,
            loading_start_time: None,
            cancel_sender: None,
        }
    }

    // fn run_watchdogs(&mut self) {
    //     watchdog_logic(self.file_path_input.value());
    //     self.output = format!(
    //         "Text: {}\nFile path: {}",
    //         self.ntfy_topic_input.value(),
    //         self.file_path_input.value()
    //     );
    //     self.show_output = true;
    //     self.loading = true;
    //     self.loading_start_time = Some(Instant::now());
    // }
    fn run_watchdogs(&mut self) {
        // Create a channel for cancellation
        let (cancel_tx, cancel_rx) = oneshot::channel();

        // Store sender for later use
        self.cancel_sender = Some(cancel_tx);

        // Clone what we need for the async task
        let file_path = self.file_path_input.value().to_string();

        // Spawn the task
        tokio::spawn(async move {
            // Create a future that completes when cancel signal is received
            let cancellation = cancel_rx.map(|_| ());

            // Create the actual work future
            let work = async {
                watchdog_logic(&file_path).await;
            };

            // Race between work and cancellation
            tokio::select! {
                _ = work => {},
                _ = cancellation => {
                    println!("Watchdog task cancelled");
                }
            }
        });

        self.output = format!(
            "Text: {}\nFile path: {}",
            self.ntfy_topic_input.value(),
            self.file_path_input.value()
        );
        self.show_output = true;
        self.loading = true;
        self.loading_start_time = Some(Instant::now());
    }
    // fn stop_watchdogs(&mut self) {
    //     self.loading = false;
    //     self.loading_start_time = None;
    // }
    fn stop_watchdogs(&mut self) {
        // Send cancellation signal if we have a sender
        if let Some(cancel_tx) = self.cancel_sender.take() {
            let _ = cancel_tx.send(());  // Ignore errors if receiver is already dropped
        }

        self.loading = false;
        self.loading_start_time = None;
    }
    fn next_input(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::NtfyTopic => InputMode::FilePathInput,
            InputMode::FilePathInput => InputMode::RunButton,
            InputMode::RunButton => InputMode::StopButton,
            InputMode::StopButton => InputMode::NtfyTopic,
        };
    }

    fn update_sparkline(&mut self) {
        if self.loading {
            // Generate random value between 0 and 100
            let between = Uniform::from(0..100);
            let value = between.sample(&mut self.rng);

            // Update sparkline data
            self.sparkline_data.pop_front();
            self.sparkline_data.push_back(value);
        }
    }
}

#[tokio::main]
async fn main() {
    // Setup terminal
    enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen); //EnableMouseCapture
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create app state
    let mut app = App::new();

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

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(8), // ASCII logo - increased height
                        Constraint::Length(3), // Text input
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

            // Ntfy topic input
            let ntfy_topic_input_style = match app.input_mode {
                InputMode::NtfyTopic => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            let text_input = Paragraph::new(app.ntfy_topic_input.value())
                .style(ntfy_topic_input_style)
                .block(Block::default().borders(Borders::ALL).title(" NTFY Topic "));
            f.render_widget(text_input, chunks[1]);

            // File path input
            let file_path_style = match app.input_mode {
                InputMode::FilePathInput => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            let file_path_input = Paragraph::new(app.file_path_input.value())
                .style(file_path_style)
                .block(Block::default().borders(Borders::ALL).title(" config.toml file path "));
            f.render_widget(file_path_input, chunks[2]);

            // Create horizontal layout for buttons
            let button_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[3]);

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
            f.render_widget(sparkline, chunks[4]);

            // Output
            if app.show_output {
                let output = Paragraph::new(app.output.clone())
                    .block(Block::default().borders(Borders::ALL).title("Result"));
                f.render_widget(output, chunks[5]);
            }
        }).unwrap();

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
                        InputMode::NtfyTopic => {
                            app.ntfy_topic_input.handle_event(&Event::Key(key));
                        }
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
