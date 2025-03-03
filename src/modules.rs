use crate::match_info::{MatchInfo, Priority, Weapon};

pub enum Modules {
    CyranoServer,
    ConsoleBackend,
    LegacyBackend,
    SlintFrontend,
    TextFrontend,
    VideoRecorder,
}

pub enum MessageType {
    Error(String),
    MatchInfoChanged(MatchInfo),
}

pub trait ModuleOperations {
    fn run(&self);
}


pub struct Message {
    pub sender: Modules,
    pub message: MessageType,
}