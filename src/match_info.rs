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

    pub p_card_l_b: bool,
    pub p_card_l_t: bool,
    pub p_card_r_b: bool,
    pub p_card_r_t: bool,

    pub auto_score_on: bool,
    pub auto_timer_on: bool,

    pub left_red_led_on: bool,
    pub left_white_led_on: bool,
    pub right_green_led_on: bool,
    pub right_white_led_on: bool,

    pub last_cyrano_request: Option<std::time::Instant>,
}

impl MatchInfo {
    pub fn new() -> Self {
        Self {
            weapon: Weapon::Unknown,
            left_score: 0,
            right_score: 0,
            timer: 300,
            period: 1,
            priority: Priority::None,
            passive_indicator: 0,
            passive_counter: 60,

            p_card_l_b: false,
            p_card_l_t: false,
            p_card_r_b: false,
            p_card_r_t: false,

            auto_score_on: false,
            auto_timer_on: false,
        
            left_red_led_on: false,
            left_white_led_on: false,
            right_green_led_on: false,
            right_white_led_on: false,
        
            last_cyrano_request: None,
        }
    }
}
