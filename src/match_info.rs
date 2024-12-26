use crate::modules::MatchInfoMessage;

pub enum Priority {
    Left,
    None,
    Right
}

pub struct MatchInfo {
    pub left_score: u32,
    pub right_score: u32,
    pub timer: u32,
    pub period: u32,
    pub priority: Priority,
    pub passive_indicator: u32,
    pub passive_counter: u32,
}

impl MatchInfo
{
    pub fn new() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            timer: 0,
            period: 0,
            priority: Priority::None,
            passive_indicator: 0,
            passive_counter: 60,
        }
    }

    pub fn match_info_changed(&mut self, msg: MatchInfoMessage) {
        match msg {
            MatchInfoMessage::LeftScoreChanged(new_data) => {self.left_score = new_data},
            MatchInfoMessage::RightScoreChanged(new_data) => {self.right_score = new_data},
            MatchInfoMessage::PeriodChanged(new_data) => {self.period = new_data}
            // MatchInfoMessage::TimerMinutesChanged(new_data) => {self.timer_minutes = new_data},
            MatchInfoMessage::TimerChanged(new_data) => {self.timer = new_data},
            MatchInfoMessage::PassiveIndicatorChanged(new_data) => {self.passive_counter = new_data},
            MatchInfoMessage::PassiveTimerChanged(new_data) => {self.passive_indicator = new_data},
        }
    }
}