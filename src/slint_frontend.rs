use slint::{Timer, TimerMode};
use std::sync::{mpsc, Arc, Mutex};

use crate::match_info::{self, Message, Priority, ProgramState, Weapon};

use crate::{layouts::*, modules};

pub struct SlintFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,

    // tx_to_main: mpsc::Sender<Message>,
    // tx_to_module: mpsc::Sender<Message>,
    // rx_to_module: mpsc::Receiver<Message>,
}

fn update_data(match_info: &Arc<Mutex<match_info::MatchInfo>>, app: &Virtuoso) {
    let match_info_data = match_info.lock().unwrap();
    app.set_left_score(match_info_data.left_score as i32);
    app.set_right_score(match_info_data.right_score as i32);
    app.set_timer(match_info_data.timer as i32);
    app.set_period(match_info_data.period as i32);

    app.set_weapon(match match_info_data.weapon {
        Weapon::Unknown => 0,
        Weapon::Epee => 1,
        Weapon::Sabre => 2,
        Weapon::Fleuret => 3,
    });

    app.set_priority(match match_info_data.priority {
        Priority::Left => -1,
        Priority::None => 0,
        Priority::Right => 1,
    });

    app.set_left_color_led_on(match_info_data.left_red_led_on);
    app.set_left_white_led_on(match_info_data.left_white_led_on);
    app.set_right_color_led_on(match_info_data.right_green_led_on);
    app.set_right_white_led_on(match_info_data.right_white_led_on);

    app.set_left_caution(match_info_data.left_caution);
    app.set_left_penalty(match_info_data.left_penalty);
    app.set_right_caution(match_info_data.right_caution);
    app.set_right_penalty(match_info_data.right_penalty);

    app.set_left_bot_pcard(match_info_data.left_pcard_bot);
    app.set_left_top_pcard(match_info_data.left_pcard_top);
    app.set_right_bot_pcard(match_info_data.right_pcard_bot);
    app.set_right_top_pcard(match_info_data.right_pcard_top);

    app.set_auto_score_on(match_info_data.auto_score_on);
    app.set_auto_timer_on(match_info_data.auto_timer_on);

    app.set_passive_counter(if match_info_data.passive_counter <= 60 {
        match_info_data.passive_counter as i32
    } else {
        -1
    });
    app.set_passive_indicator(match_info_data.passive_indicator as i32);

    app.set_is_online(match_info_data.cyrano_online);
}

impl SlintFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        // tx_to_main: mpsc::Sender<match_info::Message>,
    ) -> Self {
        // let (tx_to_module, rx_to_module) = mpsc::channel();
        Self {
            match_info,
            // tx_to_main,
            // tx_to_module,
            // rx_to_module,
        }
    }
// }

// impl modules::VirtuosoModule for SlintFrontend {
    const MODULE_TYPE: modules::Modules = modules::Modules::SlintFrontend;

    pub fn run(&mut self) {
        let app = Virtuoso::new().unwrap();

        app.set_layout(LAYOUT_1920X480);

        update_data(&self.match_info, &app);

        let weak_app_1 = app.as_weak();
        let weak_app_2 = app.as_weak();
        let timer = Timer::default();

        let match_info_clone = self.match_info.clone();

        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(100),
            move || {
                if let Some(app) = weak_app_1.upgrade() {
                    update_data(&match_info_clone, &app);
                }
            },
        );

        let flash_timer = Timer::default();
        flash_timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(500),
            move || {
                if let Some(app) = weak_app_2.upgrade() {
                    app.set_timer_flashing(!app.get_timer_flashing());
                }
            },
        );

        app.run().unwrap();

        let mut match_info_data = self.match_info.lock().unwrap();
        match_info_data.program_state = ProgramState::Exiting;
    }

    // fn get_tx_to_module(&self) -> std::sync::mpsc::Sender<Message> {
    //     self.tx_to_module.clone()
    // }
}
