use std::sync::{mpsc::{Receiver, Sender}};

pub enum Modules {
    CyranoServer,
    BackendV1,
    SlintFrontend,
    VideoRecorder,
}

pub enum MatchInfoMessage {
    LeftScoreChanged(u32),
    RightScoreChanged(u32),
    PeriodChanged(u32),
    TimerChanged(u32),
    PassiveIndicatorChanged(u32),
    PassiveTimerChanged(u32),
}

pub enum MessageType {
    Error(String),
    MatchInfoChanged(MatchInfoMessage),    
}

pub trait ModuleOperations {
    fn run(&self);
}


pub struct Message {
    pub sender: Modules,
    pub message: MessageType,
}