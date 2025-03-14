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
    fn run(&mut self);
    fn get_module_type(&self) -> Modules;
}
