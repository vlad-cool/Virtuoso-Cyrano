use std::io;
use std::sync::mpsc::RecvError;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

use slint::VecModel;

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
    Period,

    Weapon,
    
    LeftWhiteLed,
    LeftColorLed,
    RightWhiteLed,
    RightColorLed,
    
    LeftCaution,
    LeftPenalty,
    RightCaution,
    RightPenalty,

    LeftPCardBot,
    LeftPCardTop,
    RightPCardBot,
    RightPCardTop,

    Unknown,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::LeftScore => write!(f, "Left Score"),
            Field::RightScore => write!(f, "Right Score"),
            Field::Time => write!(f, "Time"),
            Field::Period => write!(f, "Period"),

            Field::Weapon => write!(f, "Weapon"),

            Field::LeftWhiteLed => write!(f, "Left White Led"),
            Field::LeftColorLed => write!(f, "Left Color Led"),
            Field::RightWhiteLed => write!(f, "Right White Led"),
            Field::RightColorLed => write!(f, "Right Color Led"),

            Field::LeftCaution => write!(f, "Left Caution"),
            Field::LeftPenalty => write!(f, "Left Penalty"),
            Field::RightCaution => write!(f, "Right Caution"),
            Field::RightPenalty => write!(f, "Right Penalty"),

            Field::LeftPCardBot => write!(f, "Left Bottom PCard"),
            Field::LeftPCardTop => write!(f, "Left Top PCard"),
            Field::RightPCardBot => write!(f, "Right Bottom PCard"),
            Field::RightPCardTop => write!(f, "Right Top PCard"),

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
        "period" => Field::Period,

        "weapon" => Field::Weapon,

        "leftwhiteled" => Field::LeftWhiteLed,
        "leftcolorled" => Field::LeftColorLed,
        "rightwhiteled" => Field::RightWhiteLed,
        "rightcolorled" => Field::RightColorLed,

        "leftcaution" => Field::LeftCaution,
        "leftpenalty" => Field::LeftPenalty,
        "rightcaution" => Field::RightCaution,
        "rightpenalty" => Field::RightPenalty,

        "leftbotpcard" => Field::LeftPCardBot,
        "lefttoppcard" => Field::LeftPCardTop,
        "rightbotpcard" => Field::RightPCardBot,
        "righttoppcard" => Field::RightPCardTop,

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
                        Field::Period => match_info_data.period = value,

                        Field::Weapon => match_info_data.weapon = match value {
                            1 => match_info::Weapon::Epee,
                            2 => match_info::Weapon::Sabre,
                            3 => match_info::Weapon::Fleuret,
                            _ => match_info::Weapon::Unknown,
                        },
                        
                        Field::LeftColorLed => match_info_data.left_red_led_on = value > 0,
                        Field::LeftWhiteLed => match_info_data.left_white_led_on = value > 0,
                        Field::RightColorLed => match_info_data.right_green_led_on = value > 0,
                        Field::RightWhiteLed => match_info_data.right_white_led_on = value > 0,
                        
                        Field::LeftCaution => match_info_data.left_caution = value > 0,
                        Field::LeftPenalty => match_info_data.left_penalty = value > 0,
                        Field::RightCaution => match_info_data.right_caution = value > 0,
                        Field::RightPenalty => match_info_data.right_penalty = value > 0,
                        
                        Field::LeftPCardBot => match_info_data.left_pcard_bot = value > 0,
                        Field::LeftPCardTop => match_info_data.left_pcard_top = value > 0,
                        Field::RightPCardBot => match_info_data.right_pcard_bot = value > 0,
                        Field::RightPCardTop => match_info_data.right_pcard_top = value > 0,
                        
                        Field::Unknown => println!("Unknown field"),
                    }
                }
                Command::Get(field) => {
                    let match_info_data = self.match_info.lock().unwrap();

                    match field {
                        Field::LeftScore => println!("{}", match_info_data.left_score),
                        Field::RightScore => println!("{}", match_info_data.right_score),
                        Field::Time => println!("{}", match_info_data.timer),
                        Field::Period => println!("{}", match_info_data.period),

                        Field::Weapon => println!("{}", match_info_data.weapon),
                        
                        Field::LeftColorLed => println!("{}", match_info_data.left_red_led_on),
                        Field::LeftWhiteLed => println!("{}", match_info_data.left_white_led_on),
                        Field::RightColorLed => println!("{}", match_info_data.right_green_led_on),
                        Field::RightWhiteLed => println!("{}", match_info_data.right_white_led_on),
                        
                        Field::LeftCaution => println!("{}", match_info_data.left_caution),
                        Field::LeftPenalty => println!("{}", match_info_data.left_penalty),
                        Field::RightCaution => println!("{}", match_info_data.right_caution),
                        Field::RightPenalty => println!("{}", match_info_data.right_penalty),
                        
                        Field::LeftPCardBot => println!("{}", match_info_data.left_pcard_bot),
                        Field::LeftPCardTop => println!("{}", match_info_data.left_pcard_top),
                        Field::RightPCardBot => println!("{}", match_info_data.right_pcard_bot),
                        Field::RightPCardTop => println!("{}", match_info_data.right_pcard_top),
                        
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
