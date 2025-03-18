use gpio_cdev::{Chip, LineRequestFlags};
use serial::{self, SerialPort};
use std::sync::mpsc::RecvError;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

// use nix::poll::*;
// use quicli::prelude::*;
// use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
// use structopt::StructOpt;
use gpio_cdev::*;

use crate::match_info;
use crate::modules;

pub struct LegacyBackend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
}

// impl modules::VirtuosoModule for LegacyBackend {
impl LegacyBackend {
    const MODULE_TYPE: modules::Modules = modules::Modules::LegacyBackend;

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel::<InputData>();

        let tx_1 = tx.clone();
        let tx_2 = tx.clone();
        let tx_3 = tx.clone();

        thread::spawn(move || {
            uart_handler(tx_1);
        });

        thread::spawn(move || {
            pins_handler(tx_2);
        });

        thread::spawn(move || {
            rc5_reciever(tx_3);
        });

        loop {
            match rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => match msg {
                    InputData::UartData(msg) => {
                        let mut match_info_data = self.match_info.lock().unwrap();

                        match_info_data.left_score = msg.score_left;
                        match_info_data.right_score = msg.score_right;

                        if msg.symbol {
                        } else {
                            match_info_data.timer = if msg.period & 0b00001111 == 0b1100 {
                                4
                            } else {
                                msg.minutes
                            } * 100
                                + msg.dec_seconds * 10
                                + msg.seconds;
                        }
                        match_info_data.period = if msg.period > 0 && msg.period < 10 {
                            msg.period
                        } else {
                            match_info_data.period
                        };
                        match_info_data.priority = match msg.period {
                            0b1110 => match_info::Priority::Right,
                            0b1111 => match_info::Priority::Left,
                            0b1011 => match_info::Priority::None,
                            _ => match match_info_data.priority {
                                match_info::Priority::Right => match_info::Priority::Right,
                                match_info::Priority::Left => match_info::Priority::Left,
                                match_info::Priority::None => match_info::Priority::None,
                            },
                        };
                    }
                    _ => todo!(),
                },
            }
            // match pins_data_rx.recv() {
            //     Err(RecvError) => {}
            //     Ok => {}
            // }
        }
    }
}

impl LegacyBackend {
    pub fn new(match_info: Arc<Mutex<match_info::MatchInfo>>) -> Self {
        Self { match_info }
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

    minutes: u32,
    dec_seconds: u32,
    seconds: u32,

    timer_sound: bool,
    score_left: u32,
    score_right: u32,
    period: u32,

    yellow_card_left: u32,
    yellow_card_right: u32,
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

            score_left: (((src[6] & 0b00010000) << 1) | (src[4] & 0b00011111)) as u32,
            score_right: (((src[7] & 0b00010000) << 1) | (src[5] & 0b00011111)) as u32,

            minutes: (src[1] & 0b11) as u32,
            dec_seconds: (src[2] & 0b00001111) as u32,
            seconds: (src[3] & 0b00001111) as u32,

            period: (src[6] & 0b00001111) as u32,

            yellow_card_left: (src[7] >> 2 & 0b00000011) as u32,
            yellow_card_right: (src[7] >> 0 & 0b00000011) as u32,
        }
    }
}

fn uart_handler(tx: mpsc::Sender<InputData>) {
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

                        tx.send(InputData::UartData(UartData::from_8bytes(buf)))
                            .unwrap();
                    }
                }
            }
        }
    }
}

struct PinsData {
    wireless: bool,          // pin 7
    recording: bool,         // pin 18
    weapon: u8,              // pin 32 * 2 + pin 36
    weapon_select_btn: bool, // pin 37
}

impl PartialEq for PinsData {
    fn eq(&self, other: &Self) -> bool {
        self.wireless == other.wireless
            && self.recording == other.recording
            && self.weapon == other.weapon
            && self.weapon_select_btn == other.weapon_select_btn
    }
}

enum IrCommands {
    TimerStartStop,

    AutoTimerOnOff,
    AutoScoreOnOff,

    LeftScoreIncrement,
    LeftScoreDecrement,
    RightScoreIncrement,
    RightScoreDecrement,

    LeftPassiveCard,
    RightPassiveCard,

    LeftPenaltyCard,
    RightPenalty,

    SecondsIncrement,
    SecondsDecrement,

    PriorityRaffle,

    SetTime,
    FlipSides,

    ChangeWeapon,

    Reset,

    PeriodIncrement,

    Unknown,
}

impl IrCommands {
    pub fn from_int(command: u32) -> Self {
        match command {
            13 => IrCommands::TimerStartStop,

            1 => IrCommands::AutoTimerOnOff,
            16 => IrCommands::AutoScoreOnOff,

            2 => IrCommands::LeftScoreIncrement,
            3 => IrCommands::LeftScoreDecrement,
            9 => IrCommands::RightScoreIncrement,
            15 => IrCommands::RightScoreDecrement,

            17 => IrCommands::LeftPassiveCard,
            18 => IrCommands::RightPassiveCard,

            4 => IrCommands::LeftPenaltyCard,
            11 => IrCommands::RightPenalty,

            14 => IrCommands::SecondsIncrement,
            6 => IrCommands::SecondsDecrement,

            12 => IrCommands::PriorityRaffle,

            7 => IrCommands::SetTime,
            0 => IrCommands::FlipSides,

            5 => IrCommands::ChangeWeapon,

            10 => IrCommands::Reset,

            8 => IrCommands::PeriodIncrement,

            _ => IrCommands::Unknown,
        }
    }
}

struct IrFrame {
    new: bool,
    address: u32,
    command: IrCommands,
}

enum InputData {
    UartData(UartData),
    PinsData(PinsData),
    IrCommand(IrFrame),
}

fn rc5_reciever(tx: mpsc::Sender<InputData>) {
    let line = crate::gpio::get_pin_by_phys_number(3).unwrap();
    let mut chip = Chip::new(format!("/dev/gpiochip{}", line.chip)).unwrap();

    let mut last_interrupt_time: u64;

    last_interrupt_time = 0;

    let mut recieve_buf: [i32; 28] = [0; 28];
    let mut index = 0;

    let mut last_toggle_value = -1;

    for event in chip
        .get_line(line.line)
        .unwrap()
        .events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "gpioevents",
        )
        .unwrap()
    {
        let event = event.unwrap();

        let mut val = match event.event_type() {
            EventType::RisingEdge => 0,
            EventType::FallingEdge => 1,
        };
        let mut count = 0;

        if event.timestamp() - last_interrupt_time > 889 * 1000 * 5 / 2 {
            recieve_buf[0] = val;
            index = 1;
            count = 0;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 3 / 2 {
            count = 2;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 1 / 2 {
            count = 1;
        }

        for i in 0..count {
            recieve_buf[index] = val;
            index += 1;

            if index == 27 {
                recieve_buf[index] = 1 - val;
                index += 1;
            }

            if index == 28 {
                for i in 0..14 {
                    if recieve_buf[i * 2] + recieve_buf[i * 2 + 1] != 1 {
                        println!("Bad buffer");
                        index = 0;
                        break;
                    }
                }
            }

            if index == 28 {
                let rc5_frame: Vec<i32> = recieve_buf.iter().step_by(2).cloned().collect();

                let toggle_bit = rc5_frame[2];

                let mut address = 0;
                let mut command = 0;

                for i in 3..8 {
                    address *= 2;
                    address += rc5_frame[i];
                }

                for i in 8..14 {
                    command *= 2;
                    command += rc5_frame[i];
                }

                println!(
                    "New: {}, Address: {}, Command: {}",
                    toggle_bit != last_toggle_value, address, command
                );

                last_toggle_value = toggle_bit;

                index = 0;
                break;
            }
        }

        last_interrupt_time = event.timestamp();
    }
}

fn pins_handler(tx: mpsc::Sender<InputData>) {
    // let mut chips = Vec::<Chip>::new();

    // for path in &["/dev/gpiochip0", "/dev/gpiochip1"] {
    //     if let Ok(chip) = Chip::new(path) {
    //         chips.push(chip);
    //     } else {
    //         println!("Failed to open chip {}", path);
    //     }
    // }

    // watched_lines = Vec!([
    //     crate::gpio::get_pin_by_phys_number(7),
    //     crate::gpio::get_pin_by_phys_number(27),
    //     crate::gpio::get_pin_by_phys_number(32),
    //     crate::gpio::get_pin_by_phys_number(36),
    //     crate::gpio::get_pin_by_phys_number(37),
    // ]);

    // // Get event handles for each line to monitor.
    // let mut evt_handles: Vec<LineEventHandle> = watched_lines
    //     .into_iter()
    //     .map(|off| {
    //         let line = chips[off.chip].get_line(off.line).unwrap();
    //         line.events(
    //             LineRequestFlags::INPUT,
    //             EventRequestFlags::BOTH_EDGES,
    //             "monitor",
    //         )
    //         .unwrap()
    //     })
    //     .collect();

    // let ownedfd: Vec<OwnedFd> = evt_handles
    //     .iter()
    //     .map(|h| unsafe { OwnedFd::from_raw_fd(h.as_raw_fd()) })
    //     .collect();

    // let mut pollfds: Vec<PollFd> = ownedfd
    //     .iter()
    //     .map(|fd| PollFd::new(fd, gpio_cdev::PollEventFlags::POLLIN | gpio_cdev::PollEventFlags::POLLPRI))
    //     .collect();

    // loop {
    //     if poll(&mut pollfds, -1)? == 0 {
    //         println!("Timeout?!?");
    //     } else {
    //         for i in 0..pollfds.len() {
    //             if let Some(revts) = pollfds[i].revents() {
    //                 let h = &mut evt_handles[i];
    //                 if revts.contains(gpio_cdev::PollEventFlags::POLLIN) {
    //                     let event = h.get_event().unwrap();
    //                     println!("[{}] {:?}", h.line().offset(), event);

    //                     let val = h.get_value().unwrap();
    //                     println!("    {}", val);
    //                 } else if revts.contains(gpio_cdev::PollEventFlags::POLLPRI) {
    //                     println!("[{}] Got a POLLPRI", h.line().offset());
    //                 }
    //             }
    //         }
    //     }
    // }
}
