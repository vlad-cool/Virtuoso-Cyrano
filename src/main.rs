// #![allow(unused)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;

use match_info::MatchInfo;

mod modules;

mod console_backend;
mod match_info;

use crate::modules::VirtuosoModule;

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod legacy_backend;
// #[cfg(feature = "legacy_backend")]
// mod gpio_lib;

#[cfg(feature = "slint_frontend")]
mod layouts;
#[cfg(feature = "slint_frontend")]
mod slint_frontend;

fn main() {
    let (tx_to_main, rx_to_main) = mpsc::channel::<match_info::Message>();

    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));

    let mut modules: Vec<Box<dyn modules::VirtuosoModule + Send>> = Vec::new();
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    #[cfg(feature = "console_backend")]
    // modules.push(Box::new(console_backend::ConsoleBackend::new(
    //     Arc::clone(&match_info),
    //     tx_to_main.clone(),
    // )));
    let mut console_backend =
        console_backend::ConsoleBackend::new(Arc::clone(&match_info), tx_to_main.clone());

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend = legacy_backend::LegacyBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "slint_frontend")]
    // modules.push(Box::new(slint_frontend::SlintFrontend::new(
    //     Arc::clone(&match_info),
    //     tx_to_main.clone(),
    // )));
    let mut slint_frontend =
        slint_frontend::SlintFrontend::new(Arc::clone(&match_info), tx_to_main.clone());

    #[cfg(feature = "cyrano_server")]
    let mut cyrano_server = cyrano_server::CyranoServer::new(Arc::clone(&match_info), tx_to_main.clone(), None);

    let slint_frontend_tx = slint_frontend.get_tx_to_module();
    let console_backend_tx = console_backend.get_tx_to_module();
    let cyrano_server_tx = cyrano_server.get_tx_to_module();

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

    // let modules: Vec<Rc<modules::Modules>> = vec![
        
    // ];

    // for mut module in modules.as_slice() {
    //     threads.push(thread::spawn(move || {
    //         module.run();
    //     }));
    // }

    loop {
        match rx_to_main.recv() {
            Ok(msg) => {

                // for module in [Box::<modules::VirtuosoModule>(slint_frontend), Box::new(console_backend)] {
                //     if module.get_module_type() == msg.sender {
                //         continue;
                //     }
                // }

                if msg.sender == modules::Modules::ConsoleBackend {
                    slint_frontend_tx.send(msg.clone());
                    cyrano_server_tx.send(msg.clone());
                }
                
                if msg.sender == modules::Modules::SlintFrontend {
                    console_backend_tx.send(msg.clone());
                    cyrano_server_tx.send(msg.clone());
                }
                
                if msg.sender == modules::Modules::CyranoServer {
                    slint_frontend_tx.send(msg.clone());
                    console_backend_tx.send(msg.clone());
                }

                if let match_info::MessageContent::Exit = msg.msg {
                    break;
                }
            }
            Err(_) => {}
        }
    }

    // for mut thread in threads {
    //     thread.join().unwrap();
    // }

    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();

    #[cfg(feature = "slint_frontend")]
    slint_frontend_thread.join().unwrap();

    #[cfg(feature = "console_backend")]
    console_backend_thread.join().unwrap();

    #[cfg(feature = "cyrano_server")]
    cyrano_server_thread.join().unwrap();
}
