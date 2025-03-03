use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use match_info::MatchInfo;
use modules::Message;

mod console_backend;
mod match_info;
mod modules;

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod legacy_backend;

#[cfg(feature = "legacy_backend")]
mod gpio_lib;

#[cfg(feature = "slint_frontend")]
mod layouts;
#[cfg(feature = "slint_frontend")]
mod slint_frontend;

fn main() {
    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));

    let (tx, rx) = channel::<Message>();
    let (tx_1, rx_1) = channel::<Message>();

    #[cfg(feature = "console_backend")]
    let mut console_backend = console_backend::ConsoleBackend::new(tx, rx, Arc::clone(&match_info));

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend = legacy_backend::LegacyBackend::new(tx, rx);

    #[cfg(feature = "slint_frontend")]
    let mut slint_frontend =
        slint_frontend::SlintFrontend::new(tx_1, rx_1, Arc::clone(&match_info));

    #[cfg(feature = "console_backend")]
    let console_backend_thread = thread::spawn(move || {
        console_backend.run();
    });

    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread = thread::spawn(move || {
        legacy_backend.run();
    });

    #[cfg(feature = "slint_frontend")]
    let slint_frontend_thread = thread::spawn(move || {
        slint_frontend.run();
    });

    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();

    #[cfg(feature = "slint_frontend")]
    slint_frontend_thread.join().unwrap();
    
    #[cfg(feature = "console_backend")]
    console_backend_thread.join().unwrap();


    // #[cfg(feature = "cyrano_server")]
    // let mut cyrano_server = cyrano_server::CyranoServer::new(tx, rx, None);
    // #[cfg(feature = "cyrano_server")]
    // let cyrano_thread = thread::spawn(move || {
    //     cyrano_server.run();
    // });

    // #[cfg(feature = "cyrano_server")]
    // cyrano_thread.join().unwrap();
}
