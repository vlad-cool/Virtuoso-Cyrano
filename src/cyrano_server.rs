use std::net::{SocketAddr, UdpSocket};
use std::{
    str::FromStr,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use crate::match_info::MatchInfo;
use crate::modules::{self, MessageType};

pub enum MODES {
    CYRANO1_0,
    CYRANO1_1,
}

pub enum MESSAGES {
    HELLO,
    NEXT,
    PREV,
    DISP,
    ACK,
    NAK,
}

pub enum STATES {
    FENCING,
    HALT,
    PAUSE,
    ENDING,
    WAITING,
}

pub struct CyranoServer {
    tx: Sender<modules::Message>,
    rx: Receiver<modules::Message>,

    udp_socket: UdpSocket,
    match_info: MatchInfo,
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

            match_info: MatchInfo::new(),
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
                Err(_e) => { }
            }

            match self.rx.try_recv() {
                Ok(message) => {
                    match message.message {
                        MessageType::Error(_) => {}

                        MessageType::MatchInfoChanged(match_info) => {
                            self.match_info.match_info_changed(match_info)
                        }
                    }
                }
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
