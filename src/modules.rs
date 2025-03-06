use crate::match_info::{MatchInfo, Priority, Weapon};

pub enum Modules {
    CyranoServer,
    ConsoleBackend,
    LegacyBackend,
    SlintFrontend,
    TextFrontend,
    VideoRecorder,
}

pub trait ModuleOperations {
    fn run(&self);
}
