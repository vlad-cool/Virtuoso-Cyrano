use std::thread;
use std::sync::mpsc::channel;

mod match_info;
mod modules;
mod cyrano_server;
mod legacy_backend;

fn main() {
    let (tx, rx) = channel();

    let mut legacy_backend = legacy_backend::LegacyBackend::new(tx, rx);
    let legacy_backend_thread = thread::spawn(move || {
        legacy_backend.run();
    });
    
    legacy_backend_thread.join().unwrap();

    // let mut cyrano_server = cyrano_server::CyranoServer::new(tx, rx, None);
    // let cyrano_thread = thread::spawn(move || {
    //     cyrano_server.run();
    // });
    
    // cyrano_thread.join().unwrap();
}
