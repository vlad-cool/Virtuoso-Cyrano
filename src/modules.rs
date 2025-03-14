use crate::match_info;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Modules {
    CyranoServer,
    ConsoleBackend,
    LegacyBackend,
    SlintFrontend,
    VideoRecorder,
}

pub trait VirtuosoModule {
    // const MODULE_TYPE: Modules;
    fn run(&mut self);
    fn get_tx_to_module(&self) -> std::sync::mpsc::Sender<match_info::Message>;
    fn get_module_type(&self) -> Modules;
}
