use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::match_info;
use crate::modules;

pub enum Protocol {
    UNKNOWN,
    CYRANO1_0,
    CYRANO1_1,
}

impl Protocol {
    pub fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN   => String::from("UNKNWN"),
            Self::CYRANO1_0 => String::from("EFP1.0"),
            Self::CYRANO1_1 => String::from("EFP1.1"),
        }
    }
}

pub enum Command {
    HELLO,
    NEXT,
    PREV,
    DISP,
    ACK,
    NAK,
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Self::HELLO => String::from("HELLO"),
            Self::NEXT => String::from("NEXT"),
            Self::PREV => String::from("PREV"),
            Self::DISP => String::from("DISP"),
            Self::ACK => String::from("ACK"),
            Self::NAK => String::from("NAK"),
        }
    }
}

struct FencerInfo {
    id: u32,
    name: String,   // 20
    nation: String, // 3
    score: u16,
    status: u8,
    yellow_card: u8,
    red_card: u8,
    light: u8,
    white_light: u8,
    medical_interventions: u8,
    reserve_introduction: u8,
    p_card: u8,
}

struct RefereeInfo {
    referee_id: u32,
    referee_name: String,   // 20
    referee_nation: String, // 3
}

struct ProtocolMessage {
    // protocol: Protocol,
    // command: Command,

    protocol: String,    // 6
    command: String,     // 6

    piste: String,       // 8
    competition: String, // 8
    phase: u16,
    poul_tab: String, // 8
    match_number: u32,
    round_number: u16,
    time: String,      // 5
    stopwatch: String, // 8
    competition_type: u8,
    weapon: match_info::Weapon,
    priority: match_info::Priority,
    state: State,

    referee_info: Option<RefereeInfo>,

    right_fencer: Option<FencerInfo>,
    left_fencer: Option<FencerInfo>,
}

impl ProtocolMessage {
    fn from_string(s: String) -> Result<Self> {
        let parts: Vec<&str> = s.split('%').collect();

        if (parts.len() < 3) {
            Err("Not enough parts in string".to_string())
        }
        else
        {
            let general_info: Vec<&str> = parts[0].split('|').collect();
            // todo!();
            Ok(Self {
                protocol: general_info[0].to_string(),
                command: general_info[1].to_string(),
                piste:  general_info[2].to_string(),
                competition:  general_info[3].to_string(),
                phase:  general_info[4].parse::<u16>().expect(0),
                poul_tab:  general_info[5].to_string(),
                match_number:  general_info[6].parse::<u32>().expect(0),
                round_number:  general_info[7].parse::<u16>().expect(0),
                time:  general_info[8].to_string(),
                stopwatch:  general_info[],
                competition_type:  general_info[],
                weapon:  general_info[],
                priority:  general_info[],
                state:  general_info[],
                referee_id:  general_info[],
                referee_name:  general_info[],
                referee_nation:  general_info[],
                right_fencer:  general_info[],
                left_fencer:  general_info[],
            })
        }

    }
}

pub enum State {
    Fencing,
    Halt,
    Pause,
    Ending,
    Waiting,
}

pub struct CyranoServer {
    tx: Sender<modules::Message>,
    rx: Receiver<modules::Message>,

    udp_socket: UdpSocket,
    match_info: match_info::MatchInfo,

    protocol: Protocol,
}

impl CyranoServer {
    pub const MODULE_TYPE: modules::Modules = modules::Modules::CyranoServer;

    pub fn new(
        tx: Sender<modules::Message>,
        rx: Receiver<modules::Message>,
        udp_port: Option<u16>,
    ) -> Self {
        Self {
            tx,
            rx,

            udp_socket: UdpSocket::bind(SocketAddr::from((
                [0, 0, 0, 0],
                udp_port.unwrap_or(50100),
            )))
            .expect("couldn't bind udp socket to address"),

            match_info: match_info::MatchInfo::new(),

            protocol: Protocol::UNKNOWN,
        }
    }

    pub fn run(&mut self) {
        self.udp_socket
            .set_nonblocking(true)
            .expect("Failed to set udp socket nonblocking");

        loop {
            let mut buf = [0u8; 512];
            match self.udp_socket.recv_from(&mut buf) {
                Ok((size, _src_addr)) => {
                    println!("Got {}", String::from_utf8(buf[0..size].to_vec()).unwrap());
                }
                Err(_e) => {}
            }

            match self.rx.try_recv() {
                Ok(message) => match message.message {
                    modules::MessageType::Error(_) => {}

                    modules::MessageType::MatchInfoChanged(match_info) => {
                        self.match_info = match_info;
                    }
                },
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    self.tx
                        .send(modules::Message {
                            sender: Self::MODULE_TYPE,
                            message: modules::MessageType::Error(
                                String::from_str("Cyrano server RX broken").unwrap(),
                            ),
                        })
                        .unwrap();
                    break;
                }
            }
        }
    }
}
