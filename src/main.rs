#[allow(dead_code)]
#[allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;

use match_info::MatchInfo;

mod modules;

mod console_backend;
mod match_info;

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod legacy_backend;
#[cfg(feature = "legacy_backend")]
mod gpio;

#[cfg(feature = "slint_frontend")]
mod layouts;
#[cfg(feature = "slint_frontend")]
mod slint_frontend;

fn main() {
    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));

    #[cfg(feature = "console_backend")]
    let mut console_backend = console_backend::ConsoleBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend = legacy_backend::LegacyBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "slint_frontend")]
    let mut slint_frontend = slint_frontend::SlintFrontend::new(Arc::clone(&match_info));

    #[cfg(feature = "cyrano_server")]
    let mut cyrano_server = cyrano_server::CyranoServer::new(Arc::clone(&match_info), None);



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

    #[cfg(feature = "cyrano_server")]
    let cyrano_server_thread = thread::spawn(move || {
        cyrano_server.run();
    });

    

    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();

    #[cfg(feature = "slint_frontend")]
    slint_frontend_thread.join().unwrap();

    #[cfg(feature = "console_backend")]
    console_backend_thread.join().unwrap();

    #[cfg(feature = "cyrano_server")]
    cyrano_server_thread.join().unwrap();
}
