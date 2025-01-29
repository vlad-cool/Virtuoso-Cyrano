use serial::{self, SerialPort};
use std::sync::mpsc::RecvError;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

use crate::match_info;
use crate::modules;

pub struct LegacyBackend {
    tx: mpsc::Sender<modules::Message>,
    rx: mpsc::Receiver<modules::Message>,

    // match_info: match_info::MatchInfo,
    match_info: Mutex::<match_info::MatchInfo>,
}

impl LegacyBackend {
    pub const MODULE_TYPE: modules::Modules = modules::Modules::LegacyBackend;

    pub fn new(tx: mpsc::Sender<modules::Message>, rx: mpsc::Receiver<modules::Message>, match_info: Mutex::<match_info::MatchInfo>) -> Self {
        Self {
            tx,
            rx,
            match_info,
        }
    }

    pub fn run(&mut self) {
        let (uart_data_tx, uart_data_rx) = mpsc::channel::<UartData>();
        let (pins_data_tx, pins_data_rx) = mpsc::channel::<PinsData>();

        thread::spawn(move || {
            uart_handler(&uart_data_tx);
        });

        thread::spawn(move || {
            pins_handler(&pins_data_tx);
        });

        loop {
            match uart_data_rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => println!("{}", msg),
            }
            match pins_data_rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => println!("{}", msg),
            }
            
        }
    }
}

#[derive(Debug)]
struct UartData {
    yellow_red: bool,
    white_red: bool,
    red: bool,

    yellow_green: bool,
    white_green: bool,
    green: bool,

    apparel_sound: bool,

    symbol: bool,

    on_timer: bool,

    minutes: u8,
    dec_seconds: u8,
    seconds: u8,

    timer_sound: bool,
    score_left: u8,
    score_right: u8,
    period: Option<u8>,

    yellow_card_left: u8,
    yellow_card_right: u8,
}

impl UartData {
    fn from_8bytes(src: [u8; 8]) -> Self {
        UartData {
            yellow_red: src[0] >> 4 & 1 == 1,
            red: src[0] >> 3 & 1 == 1,
            white_green: src[0] >> 2 & 1 == 1,
            yellow_green: src[0] >> 1 & 1 == 1,
            green: src[0] >> 0 & 1 == 1,
            white_red: src[1] >> 4 & 1 == 1,
            apparel_sound: src[1] >> 3 & 1 == 1,
            symbol: src[1] >> 2 & 1 == 1,
            on_timer: src[2] >> 4 & 1 == 1,
            timer_sound: src[3] >> 4 & 1 == 1,

            score_left: ((src[6] & 0b00010000) << 1) | (src[4] & 0b00011111),
            score_right: ((src[7] & 0b00010000) << 1) | (src[5] & 0b00011111),

            minutes: if src[6] & 0b00001111 == 0b1100 {
                4
            } else {
                src[1] & 0b11
            },
            dec_seconds: src[2] & 0b00001111,
            seconds: src[3] & 0b00001111,

            period: if 0 < src[6] & 0b00001111 && src[6] & 0b00001111 < 10 {Some(src[6] & 0b00001111)} else {None},

            yellow_card_left: src[7] >> 2 & 0b00000011,
            yellow_card_right: src[7] >> 0 & 0b00000011,
        }
    }
}

fn uart_handler(tx: &mpsc::Sender<UartData>) {
    let mut port = serial::open("/dev/ttyS2").unwrap();

    let settings = serial::PortSettings {
        baud_rate: serial::BaudRate::Baud38400,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&settings).unwrap();
    port.set_timeout(Duration::from_secs(60)).unwrap();

    let mut buf: [u8; 8] = [0; 8];
    let mut ind: usize = 0;

    for byte in port.bytes() {
        match byte {
            Err(_) => {}
            Ok(byte_val) => {
                if byte_val >> 5 == 0 {
                    ind = 0;
                }

                if byte_val >> 5 == ind as u8 {
                    buf[ind] = byte_val;
                    ind += 1;

                    if ind == 8 {
                        ind = 0;

                        tx.send(UartData::from_8bytes(buf)).unwrap();
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for UartData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
"UartMessage {{ 
    yellow_red:        {}
    white_red:         {} 
    red:               {} 
    yellow_green:      {} 
    white_green:       {} 
    green:             {} 
    apparel_sound:     {} 
    symbol:            {} 
    on_timer:          {} 
    minutes:           {} 
    dec_seconds:       {} 
    seconds:           {} 
    timer_sound:       {} 
    score_left:        {} 
    score_right:       {} 
    period:            {} 
    yellow_card_left:  {} 
    yellow_card_right: {} 
}}", 
        self.yellow_red,
        self.white_red,
        self.red,
        self.yellow_green,
        self.white_green,
        self.green,
        self.apparel_sound,
        self.symbol,
        self.on_timer,
        self.minutes,
        self.dec_seconds,
        self.seconds,
        self.timer_sound,
        self.score_left,
        self.score_right,
        match self.period {
            Some(val) => val,
            None => 255,
        },
        self.yellow_card_left,
        self.yellow_card_right,
    )
    }
}

struct PinsData {
    wireless: bool, // pin 7
    recording: bool, // pin 18
    weapon: u8, // pin 32 * 2 + pin 36
    weapon_select_btn: bool, // pin 37
}

impl PartialEq for PinsData {
    fn eq(&self, other: &Self) -> bool {
        self.wireless == other.wireless &&
        self.recording == other.recording &&
        self.weapon == other.weapon &&
        self.weapon_select_btn == other.weapon_select_btn
    }
}

impl std::fmt::Display for PinsData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
"Pins {{ 
    wireless:          {}
    recording:         {} 
    weapon:            {} 
    weapon_select_btn: {} 
}}", 
        self.wireless,
        self.recording,
        self.weapon,
        self.weapon_select_btn,
    )
    }
}

fn pins_handler(tx: &mpsc::Sender<PinsData>) {
    loop {
        
        thread::sleep(Duration::from_millis(10));
    }
}
