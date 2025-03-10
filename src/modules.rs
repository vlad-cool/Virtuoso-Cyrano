use crate::match_info;

pub enum Modules {
    CyranoServer,
    ConsoleBackend,
    LegacyBackend,
    SlintFrontend,
    TextFrontend,
    VideoRecorder,
}

pub trait VirtuosoModule {
    const MODULE_TYPE: Modules;
    fn run(&mut self);
    fn get_tx_to_module(&self) -> std::sync::mpsc::Sender<match_info::Message>;
}
