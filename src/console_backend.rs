use std::io;
use std::sync::mpsc::RecvError;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

use crate::match_info;
use crate::modules;

pub struct ConsoleBackend {
    tx: mpsc::Sender<modules::Message>,
    rx: mpsc::Receiver<modules::Message>,

    match_info: Arc<Mutex<match_info::MatchInfo>>,
}

#[derive(Debug)]
enum Field {
    LeftScore,
    RightScore,
    Time,
    Unknown,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::LeftScore => write!(f, "Left Score"),
            Field::RightScore => write!(f, "Right Score"),
            Field::Time => write!(f, "Time"),
            Field::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
enum Command {
    Set(Field, u32),
    Get(Field),
    Unknown,
}

fn parse_field(input: &str) -> Field {
    match input {
        "leftscore" => Field::LeftScore,
        "rightscore" => Field::RightScore,
        "time" => Field::Time,
        _ => Field::Unknown,
    }
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["set", variable, value] => match parse_field(&variable) {
            Field::Unknown => Command::Unknown,
            field => {
                match value.parse::<u32>() {
                    Ok(value) => Command::Set(field, value),
                    _ => Command::Unknown,
                }
            }
        },
        ["get", variable] => match parse_field(&variable) {
            Field::Unknown => Command::Unknown,
            field => Command::Get(field),
        },
        _ => Command::Unknown,
    }
}

impl ConsoleBackend {
    pub const MODULE_TYPE: modules::Modules = modules::Modules::ConsoleBackend;

    pub fn new(
        tx: mpsc::Sender<modules::Message>,
        rx: mpsc::Receiver<modules::Message>,
        match_info: Arc<Mutex<match_info::MatchInfo>>,
    ) -> Self {
        Self { tx, rx, match_info }
    }

    pub fn run(&mut self) {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_ascii_lowercase();

            if input == "" {
                continue;
            }

            let command = parse_command(&input);

            match command {
                Command::Set(field, value) => {
                    let mut match_info_data = self.match_info.lock().unwrap();
                    
                    match field {
                        Field::LeftScore => match_info_data.left_score = value,
                        Field::RightScore => match_info_data.right_score = value,
                        Field::Time => match_info_data.timer = value,
                        Field::Unknown => println!("Unknown field"),
                    }
                }
                Command::Get(field) => {
                    let match_info_data = self.match_info.lock().unwrap();

                    match field {
                        Field::LeftScore => println!("{}", match_info_data.left_score),
                        Field::RightScore => println!("{}", match_info_data.right_score),
                        Field::Time => println!("{}", match_info_data.timer),
                        Field::Unknown => println!("Unknown field"),
                    }
                }
                Command::Unknown => {
                    println!("Unknown command or invalid format");
                }
            }
        }
    }
}
