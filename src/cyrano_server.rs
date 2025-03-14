use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Instant;

use crate::match_info::{self, MatchInfo};
use crate::modules::{self, VirtuosoModule};

enum Protocol {
    UNKNOWN,
    CYRANO1_0,
    CYRANO1_1,
}

impl Protocol {
    pub fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNWN"),
            Self::CYRANO1_0 => String::from("EFP1.0"),
            Self::CYRANO1_1 => String::from("EFP1.1"),
        }
    }
}

// pub enum State {
//     Fencing,
//     Halt,
//     Pause,
//     Ending,
//     Waiting,
// }

struct FencerInfo {
    id: String,     //8
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

impl FencerInfo {
    pub fn new() -> Self {
        Self {
            id: String::from_str("").unwrap(),
            name: String::from_str("").unwrap(),   // 20
            nation: String::from_str("").unwrap(), // 3
            score: 0,
            status: 0,
            yellow_card: 0,
            red_card: 0,
            light: 0,
            white_light: 0,
            medical_interventions: 0,
            reserve_introduction: 0,
            p_card: 0,
        }
    }
    //     fn from_string(s: String) -> Result<Self, String> {
    //         let parts: Vec<&str> = s.split('|').collect();

    //         if parts.len() != 13 {
    //             Err("Wrong number of elements in input string with fencer info".to_string())
    //         } else {
    //             // let general_info: Vec<&str> = parts[0].split('|').collect();
    //             // todo!();
    //             // Err("Not implemented".to_string()) // TODO
    //             Ok(Self {
    //                 id: parts[1],
    //                 name: parts[2],   // 20
    //                 nation: parts[3], // 3
    //                 score: 0,
    //                 status: 0,
    //                 yellow_card: 0,
    //                 red_card: 0,
    //                 light: 0,
    //                 white_light: 0,
    //                 medical_interventions: 0,
    //                 reserve_introduction: 0,
    //                 p_card: 0,
    //             })
    //         }
    //     }

    pub fn to_1_0_string(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.id,
            self.name,
            self.nation,
            self.score,
            self.status,
            self.yellow_card,
            self.red_card,
            self.light,
            self.white_light,
            self.medical_interventions,
            self.reserve_introduction
        )
    }
    pub fn to_1_1_string(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.id,
            self.name,
            self.nation,
            self.score,
            self.status,
            self.yellow_card,
            self.red_card,
            self.light,
            self.white_light,
            self.medical_interventions,
            self.reserve_introduction,
            self.p_card
        )
    }
}

// struct RefereeInfo {
//     referee_id: u32,
//     referee_name: String,   // 20
//     referee_nation: String, // 3
// }

// struct ProtocolMessage {
//     protocol: Protocol,
//     command: String,

//     // protocol: String,    // 6
//     // command: String,     // 6
//     piste: String,       // 8
//     competition: String, // 8
//     phase: u16,
//     poul_tab: String, // 8
//     match_number: u32,
//     round_number: u16,
//     time: String,      // 5
//     stopwatch: String, // 8
//     competition_type: u8,
//     weapon: match_info::Weapon,
//     priority: match_info::Priority,
//     state: State,

//     referee_info: Option<RefereeInfo>,

//     right_fencer: Option<FencerInfo>,
//     left_fencer: Option<FencerInfo>,
// }

// impl ProtocolMessage {
//     fn from_string(s: String) -> Result<Self, String> {
//         let parts: Vec<&str> = s.split('%').collect();

//         if (parts.len() < 3) {
//             Err("Not enough parts in string".to_string())
//         }
//         else
//         {
//             let general_info: Vec<&str> = parts[0].split('|').collect();
//             // todo!();
//             // Err("Not implemented".to_string()) // TODO
//             Ok(Self {
//                 protocol: general_info[0].to_string(),
//                 command: general_info[1].to_string(),
//                 piste:  general_info[2].to_string(),
//                 competition:  general_info[3].to_string(),
//                 phase:  general_info[4].parse::<u16>().expect(0),
//                 poul_tab:  general_info[5].to_string(),
//                 match_number:  general_info[6].parse::<u32>().expect(0),
//                 round_number:  general_info[7].parse::<u16>().expect(0),
//                 time:  general_info[8].to_string(),
//                 stopwatch:  general_info[],
//                 competition_type:  general_info[],
//                 weapon:  general_info[],
//                 priority:  general_info[],
//                 state:  general_info[],
//                 // referee_id:  general_info[],
//                 // referee_name:  general_info[],
//                 // referee_nation:  general_info[],
//                 referee_info: general_info[]
//                 right_fencer:  general_info[],
//                 left_fencer:  general_info[],
//             })
//         }

//     }
// }

pub struct CyranoServer {
    tx_to_main: mpsc::Sender<match_info::Message>,
    tx_to_module: mpsc::Sender<match_info::Message>,
    rx_to_module: mpsc::Receiver<match_info::Message>,
    
    match_info: Arc<Mutex<match_info::MatchInfo>>,

    prev_match_info: MatchInfo,

    udp_socket: UdpSocket,

    protocol: Protocol,
    software_ip: Option<SocketAddr>,

    last_hello: Option<Instant>,
    online: bool,

    left_fencer: FencerInfo,
    right_fencer: FencerInfo,
}

impl VirtuosoModule for CyranoServer {
    // const MODULE_TYPE: modules::Modules = modules::Modules::CyranoServer;

    fn run(&mut self) {
        self.udp_socket
            .set_nonblocking(true)
            .expect("Failed to set udp socket nonblocking");

        loop {
            let mut buf = [0u8; 512];
            match self.udp_socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => {
                    println!("Got {}", String::from_utf8(buf[0..size].to_vec()).unwrap());

                    let sss: String = String::from_utf8(buf.to_vec()).unwrap();

                    let parts: Vec<&str> = sss.split("|").collect();

                    println!("parts[2]: {}", parts[2]);

                    self.protocol = match parts[1] {
                        "EFP1" => Protocol::CYRANO1_0,
                        "EFP1.1" => Protocol::CYRANO1_1,
                        _ => Protocol::UNKNOWN,
                    };

                    self.software_ip = Some(src_addr);

                    match parts[2] {
                        "HELLO" => {
                            self.last_hello = Some(Instant::now());

                            self.send_full_info();
                        }
                        _ => {}
                    }
                }
                Err(_e) => {}
            }

            // match match_info_data.program_state {
            //     match_info::ProgramState::Exiting => break,
            //     _ => {}
            // }

            let mut data_updated = false;

            {
                let match_info_data = self.match_info.lock().unwrap();

                if self.prev_match_info.weapon != match_info_data.weapon
                    || self.prev_match_info.left_score != match_info_data.left_score
                    || self.prev_match_info.right_score != match_info_data.right_score
                    || self.prev_match_info.timer != match_info_data.timer
                    || self.prev_match_info.period != match_info_data.period
                    || self.prev_match_info.priority != match_info_data.priority
                    || self.prev_match_info.passive_indicator != match_info_data.passive_indicator
                    || self.prev_match_info.passive_counter != match_info_data.passive_counter
                    || self.prev_match_info.auto_score_on != match_info_data.auto_score_on
                    || self.prev_match_info.auto_timer_on != match_info_data.auto_timer_on
                    || self.prev_match_info.left_red_led_on != match_info_data.left_red_led_on
                    || self.prev_match_info.left_white_led_on != match_info_data.left_white_led_on
                    || self.prev_match_info.right_green_led_on != match_info_data.right_green_led_on
                    || self.prev_match_info.right_white_led_on != match_info_data.right_white_led_on
                    || self.prev_match_info.left_caution != match_info_data.left_caution
                    || self.prev_match_info.left_penalty != match_info_data.left_penalty
                    || self.prev_match_info.right_caution != match_info_data.right_caution
                    || self.prev_match_info.right_penalty != match_info_data.right_penalty
                    || self.prev_match_info.left_pcard_bot != match_info_data.left_pcard_bot
                    || self.prev_match_info.left_pcard_top != match_info_data.left_pcard_top
                    || self.prev_match_info.right_pcard_bot != match_info_data.right_pcard_bot
                    || self.prev_match_info.right_pcard_top != match_info_data.right_pcard_top
                {
                    self.left_fencer.score = match_info_data.left_score as u16;
                    self.right_fencer.score = match_info_data.right_score as u16;
                    // self.prev_match_info = match_info_data;

                    self.prev_match_info.weapon = match_info_data.weapon;
                    self.prev_match_info.left_score = match_info_data.left_score;
                    self.prev_match_info.right_score = match_info_data.right_score;
                    self.prev_match_info.timer = match_info_data.timer;
                    self.prev_match_info.period = match_info_data.period;
                    self.prev_match_info.priority = match_info_data.priority;
                    self.prev_match_info.passive_indicator = match_info_data.passive_indicator;
                    self.prev_match_info.passive_counter = match_info_data.passive_counter;
                    self.prev_match_info.auto_score_on = match_info_data.auto_score_on;
                    self.prev_match_info.auto_timer_on = match_info_data.auto_timer_on;
                    self.prev_match_info.left_red_led_on = match_info_data.left_red_led_on;
                    self.prev_match_info.left_white_led_on = match_info_data.left_white_led_on;
                    self.prev_match_info.right_green_led_on = match_info_data.right_green_led_on;
                    self.prev_match_info.right_white_led_on = match_info_data.right_white_led_on;
                    self.prev_match_info.left_caution = match_info_data.left_caution;
                    self.prev_match_info.left_penalty = match_info_data.left_penalty;
                    self.prev_match_info.right_caution = match_info_data.right_caution;
                    self.prev_match_info.right_penalty = match_info_data.right_penalty;
                    self.prev_match_info.left_pcard_bot = match_info_data.left_pcard_bot;
                    self.prev_match_info.left_pcard_top = match_info_data.left_pcard_top;
                    self.prev_match_info.right_pcard_bot = match_info_data.right_pcard_bot;
                    self.prev_match_info.right_pcard_top = match_info_data.right_pcard_top;

                    data_updated = true;
                }
            }

            if data_updated {
                self.send_full_info();
            }

            if let Some(last_hello) = self.last_hello {
                if Instant::now().duration_since(last_hello).as_secs() > 15 && self.online == true {
                    let mut match_info_data = self.match_info.lock().unwrap();
                    self.online = false;
                    match_info_data.cyrano_online = false;
                } else if Instant::now().duration_since(last_hello).as_secs() <= 15
                    && self.online == false
                {
                    let mut match_info_data = self.match_info.lock().unwrap();
                    self.online = true;
                    match_info_data.cyrano_online = true;
                }
            }
        }
    }


    fn get_tx_to_module(&self) -> std::sync::mpsc::Sender<match_info::Message> {
        self.tx_to_module.clone()
    }

    fn get_module_type(&self) -> modules::Modules {
        modules::Modules::CyranoServer
    }
}

impl CyranoServer {
    pub fn new(match_info: Arc<Mutex<match_info::MatchInfo>>, tx_to_main: mpsc::Sender<match_info::Message>, udp_port: Option<u16>) -> Self {
        let (tx_to_module, rx_to_module) = mpsc::channel();
        Self {
            match_info: match_info,
            tx_to_main,
            tx_to_module,
            rx_to_module,
            udp_socket: UdpSocket::bind(SocketAddr::from((
                [0, 0, 0, 0],
                udp_port.unwrap_or(50100),
            )))
            .expect("couldn't bind udp socket to address"),

            protocol: Protocol::UNKNOWN,

            software_ip: None,

            last_hello: None,

            online: false,

            left_fencer: FencerInfo::new(),
            right_fencer: FencerInfo::new(),

            prev_match_info: MatchInfo::new(),
        }
    }

    fn send_full_info(&self) {
        let match_info_data = self.match_info.lock().unwrap();
        let buf = format!(
            "|{}|INFO|7|aboba|2|7|8|2||{}||{}|{}|E||||%{}%{}",
            self.protocol.to_string(),
            match_info_data.timer,
            match_info_data.weapon,
            match_info_data.priority,
            match self.protocol {
                Protocol::UNKNOWN => String::from(""),
                Protocol::CYRANO1_0 => self.right_fencer.to_1_0_string(),
                Protocol::CYRANO1_1 => self.right_fencer.to_1_1_string(),
            },
            match self.protocol {
                Protocol::UNKNOWN => String::from(""),
                Protocol::CYRANO1_0 => self.left_fencer.to_1_0_string(),
                Protocol::CYRANO1_1 => self.left_fencer.to_1_1_string(),
            },
        );
        println!("{}", buf);
        if let Some(dest_ip) = self.software_ip {
            match self.udp_socket.send_to(buf.as_bytes(), dest_ip) {
                Ok(_) => {}
                Err(e) => println!("Failed to send UDP packet, error {}", e),
            }
        }
    }
}
