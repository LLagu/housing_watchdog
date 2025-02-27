use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

use crate::watchdog_logic;
use futures::FutureExt;
use std::{collections::VecDeque, error::Error, io, time::Instant};
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
    pub(crate) show_output: bool,
    pub(crate) loading: bool,
    pub(crate) sparkline_data: VecDeque<u64>,
    rng: ThreadRng,
    loading_start_time: Option<Instant>,
    cancel_sender: Option<oneshot::Sender<()>>,
}

impl App {
    pub(crate) fn new() -> App {
        // Initialize with some random data to fill the sparkline
        let mut initial_data = VecDeque::with_capacity(100);
        let mut rng = rand::thread_rng();
        // let between = Uniform::from(0..100);

        for _ in 0..100 {
            // initial_data.push_back(between.sample(&mut rng));
            initial_data.push_back(0);
        }

        App {
            file_path_input: Input::default(),
            input_mode: InputMode::FilePathInput,
            output: String::new(),
            show_output: false,
            loading: false,
            sparkline_data: initial_data,
            rng,
            loading_start_time: None,
            cancel_sender: None,
        }
    }

    pub(crate) fn run_watchdogs(&mut self) {
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

        self.output = format!("File path: {}", self.file_path_input.value());
        self.show_output = true;
        self.loading = true;
        self.loading_start_time = Some(Instant::now());
    }

    pub(crate) fn stop_watchdogs(&mut self) {
        // Send cancellation signal if we have a sender
        if let Some(cancel_tx) = self.cancel_sender.take() {
            let _ = cancel_tx.send(()); // Ignore errors if receiver is already dropped
        }

        self.loading = false;
        self.loading_start_time = None;
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
            // Generate random value between 0 and 100
            let between = Uniform::from(0..100);
            let value = between.sample(&mut self.rng);

            // Update sparkline data
            self.sparkline_data.pop_front();
            self.sparkline_data.push_back(value);
        }
    }
}
