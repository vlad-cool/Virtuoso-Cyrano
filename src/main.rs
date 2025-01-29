use std::thread;
use std::sync::mpsc::channel;
use std::sync::Mutex;

use match_info::MatchInfo;
use modules::Message;

mod match_info;
mod modules;

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod legacy_backend;

#[cfg(feature = "legacy_backend")]
mod gpio_lib;

fn main() {
    let match_info: Mutex<MatchInfo> = Mutex::<MatchInfo>::new(MatchInfo::new());

    let (tx, rx) = channel::<Message>();

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend = legacy_backend::LegacyBackend::new(tx, rx);
    
    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread = thread::spawn(move || {
        legacy_backend.run();
    });
    
    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();


    #[cfg(feature = "cyrano_server")]
    let mut cyrano_server = cyrano_server::CyranoServer::new(tx, rx, None);
    #[cfg(feature = "cyrano_server")]
    let cyrano_thread = thread::spawn(move || {
        cyrano_server.run();
    });
    

    #[cfg(feature = "cyrano_server")]
    cyrano_thread.join().unwrap();
}
