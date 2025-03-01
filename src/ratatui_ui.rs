use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

use crate::session::{get_prev_session_file_path, PrevSessionFileType};
use crate::watchdog_logic;
use futures::FutureExt;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::{collections::VecDeque, time::Instant};
use tokio::sync::oneshot;
use tui_input::Input;

pub(crate) enum InputMode {
    FilePathInput,
    RunButton,
    StopButton,
}

pub(crate) struct App {
    pub(crate) file_path_input: Input,
    pub(crate) input_mode: InputMode,
    pub(crate) output: String,
    pub(crate) loading: bool,
    pub(crate) sparkline_data: VecDeque<u64>,
    rng: ThreadRng,
    cancel_sender: Option<oneshot::Sender<()>>,
}

impl App {
    pub(crate) fn new() -> App {
        // Initialize with some random data to fill the sparkline
        let mut initial_data = VecDeque::with_capacity(100);
        let mut rng = rand::thread_rng();

        for _ in 0..200 {
            initial_data.push_back(0);
        }

        App {
            file_path_input: Input::from(load_prev_config_path()),
            input_mode: InputMode::FilePathInput,
            output: String::new(),
            loading: false,
            sparkline_data: initial_data,
            rng,
            cancel_sender: None,
        }
    }

    pub(crate) fn run_watchdogs(&mut self) {
        // Create a channel for cancellation
        let (cancel_tx, cancel_rx) = oneshot::channel();

        // Store sender for later use
        self.cancel_sender = Some(cancel_tx);

        // Clone what we need for the async task
        let mut file_path = self.file_path_input.value().to_string();
        file_path.retain(|c| !c.is_ascii_whitespace());
        save_prev_config_path(&file_path);

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

        self.output = format!("Scraping websites...");
        self.loading = true;
    }

    pub(crate) fn stop_watchdogs(&mut self) {
        // Send cancellation signal if we have a sender
        if let Some(cancel_tx) = self.cancel_sender.take() {
            let _ = cancel_tx.send(()); // Ignore errors if receiver is already dropped
        }

        self.output = format!("Scraping stopped.");
        self.loading = false;
    }
    pub(crate) fn next_input(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::FilePathInput => InputMode::RunButton,
            InputMode::RunButton => InputMode::StopButton,
            InputMode::StopButton => InputMode::FilePathInput,
        };
    }

    pub(crate) fn update_sparkline(&mut self) {
        if self.loading {
            // Generate random value from range
            let between = Uniform::from(20..100);
            let value = between.sample(&mut self.rng);

            // Update sparkline data
            self.sparkline_data.pop_back();
            self.sparkline_data.push_front(value);
        }
    }
}

fn save_prev_config_path(content: &String) {
    let prev_config_path = get_prev_session_file_path(PrevSessionFileType::ConfigPath);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(prev_config_path)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap()
}

fn load_prev_config_path() -> String {
    let prev_config_path = get_prev_session_file_path(PrevSessionFileType::ConfigPath);
    match read_to_string(&prev_config_path) {
        Ok(contents) => contents,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                File::create(prev_config_path);
                String::from("")
            }
            _ => {
                println!("{:?}", e);
                String::from("")
            }
        },
    }
}
