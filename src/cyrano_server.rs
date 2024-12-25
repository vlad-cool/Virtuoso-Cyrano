use std::sync::{mpsc::{Receiver, Sender}};
use std::net::{SocketAddr, UdpSocket};

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
    tx: Sender<MESSAGES>,
    rx: Receiver<MESSAGES>,

    // udp_port: u16,
    udp_socket: UdpSocket,

    // last_hello: Option<>;
}

impl CyranoServer {
    pub fn new(tx: Sender<MESSAGES>, rx: Receiver<MESSAGES>, udp_port: Option<u16>) -> Self {
        Self {
            tx: tx, 
            rx: rx, 

            udp_socket: UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], udp_port.unwrap_or(50100)))).expect("couldn't bind udp socket to address")
        }
    }

    pub fn run(&mut self) {
        self.udp_socket.set_nonblocking(true).expect("Failed to set udp socket nonblocking");

        loop {
            let mut buf = [0u8; 512];
            match self.udp_socket.recv_from(&mut buf) {
                Ok ((size, _src_addr)) => {
                    println!("Got {:?}", buf[0..size].to_vec());
                    println!("Got {}", String::from_utf8(buf[0..size].to_vec()).unwrap());
                }
                Err(_e) => {
                    println!("ASSDASD");
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
    }
}