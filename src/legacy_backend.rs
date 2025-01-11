use serial::{self, SerialPort};
use std::sync::mpsc::RecvError;
use std::thread;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

use crate::match_info;
use crate::modules;

pub struct LegacyBackend {
    tx: mpsc::Sender<modules::Message>,
    rx: mpsc::Receiver<modules::Message>,

    match_info: match_info::MatchInfo,
}

impl LegacyBackend {
    pub const MODULE_TYPE: modules::Modules = modules::Modules::LegacyBackend;

    pub fn new(tx: mpsc::Sender<modules::Message>, rx: mpsc::Receiver<modules::Message>) -> Self {
        Self {
            tx,
            rx,

            match_info: match_info::MatchInfo::new(),
        }
    }

    pub fn run(&mut self) {
        let (uart_data_tx, uart_data_rx) = mpsc::channel::<UartMessage>();

        let uart_handler_thread = thread::spawn(move || {
            uart_handler(&uart_data_tx);
        });

        println!("ABOBA");

        loop {
            match uart_data_rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => println!("{:?}", msg),
            }
            
        }

        uart_handler_thread.join().unwrap_err();
    }
}

#[derive(Debug)]
struct UartMessage {
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
    period: u8,

    yellow_card_left: u8,
    yellow_card_right: u8,
}

impl UartMessage {
    fn from_8bytes(src: [u8; 8]) -> Self {
        UartMessage {
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

            period: src[6] & 0b00001111,

            yellow_card_left: src[7] >> 2 & 0b00000011,
            yellow_card_right: src[7] >> 0 & 0b00000011,
        }
    }
}

fn uart_handler(tx: &mpsc::Sender<UartMessage>) {
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

                        tx.send(UartMessage::from_8bytes(buf)).unwrap();
                    }
                }
            }
        }
    }
}
