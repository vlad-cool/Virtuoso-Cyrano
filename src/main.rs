use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;

mod cyrano_server;

fn main() {
    let (tx, rx) = channel();

    let mut cyrano_server = cyrano_server::CyranoServer::new(tx, rx, None);
    let cyrano_thread = thread::spawn(move || {
        cyrano_server.run();
    });
    
    cyrano_thread.join().unwrap();
}
