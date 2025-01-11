pub enum Priority {
    Left,
    None,
    Right,
}

pub enum Weapon {
    Unknown,
    Epee,
    Sabre,
    Fleuret,
}

pub struct MatchInfo {
    pub weapon: Weapon,
    pub left_score: u32,
    pub right_score: u32,
    pub timer: u32,
    pub period: u32,
    pub priority: Priority,
    pub passive_indicator: u32,
    pub passive_counter: u32,
}

impl MatchInfo {
    pub fn new() -> Self {
        Self {
            weapon: Weapon::Unknown,
            left_score: 0,
            right_score: 0,
            timer: 0,
            period: 0,
            priority: Priority::None,
            passive_indicator: 0,
            passive_counter: 60,
        }
    }
}
