use slint::{Timer, TimerMode};
use std::clone;
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc, Mutex};

use crate::match_info::{self, Message};
use crate::modules;

use crate::layouts::*;

pub struct SlintFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    tx_to_main: mpsc::Sender<match_info::Message>,
    message_handler: Arc<Mutex<MessageHandler>>,
}

impl SlintFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        tx_to_main: mpsc::Sender<match_info::Message>,
    ) -> Self {
        // let (tx_to_module, rx_to_module) = mpsc::channel();
        Self {
            match_info: match_info,
            tx_to_main: tx_to_main,
            message_handler: Arc::new(Mutex::new(MessageHandler::new())),
        }
    }
}

impl modules::VirtuosoModule for SlintFrontend {
    // const MODULE_TYPE: modules::Modules = modules::Modules::SlintFrontend;

    fn run(&mut self) {
        let app = Virtuoso::new().unwrap();

        app.set_layout(LAYOUT_1920X480);

        // let rx_to_module = self.rx_to_module;

        // self.update_data(Weak(self.rx_to_module), &self.match_info, &app);

        let weak_app_1 = app.as_weak();
        let weak_app_2 = app.as_weak();
        let timer = Timer::default();

        let match_info_clone = self.match_info.clone();

        let message_handler_1 = self.message_handler.clone();

        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(100),
            move || {
                if let Some(app) = weak_app_1.upgrade() {
                    message_handler_1.lock().unwrap().update_data(&match_info_clone, &app);
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
        match_info_data.program_state = match_info::ProgramState::Exiting;
    }

    fn get_tx_to_module(&self) -> std::sync::mpsc::Sender<match_info::Message> {
        self.message_handler.lock().unwrap().tx_to_module.clone()
    }

    fn get_module_type(&self) -> modules::Modules {
        modules::Modules::SlintFrontend
    }
}

impl SlintFrontend {}

struct MessageHandler {
    tx_to_module: mpsc::Sender<match_info::Message>,
    rx_to_module: mpsc::Receiver<match_info::Message>,
}

impl MessageHandler {
    pub fn new() -> Self {
        let (tx_to_module, rx_to_module) = mpsc::channel();
        Self {
            tx_to_module,
            rx_to_module,
        }
    }

    pub fn update_data(&self, match_info: &Arc<Mutex<match_info::MatchInfo>>, app: &Virtuoso) {
        match self.rx_to_module.try_recv() {
            Ok(msg) => {
                match msg.msg {
                    match_info::MessageContent::MatchInfoUpdated => {}
                    match_info::MessageContent::Exit => {return} // TODO good thread exit
                }
            }
            Err(err) => match err {
                TryRecvError::Disconnected => return,
                TryRecvError::Empty => return,
            },
        }

        let match_info_data = match_info.lock().unwrap();
        app.set_left_score(match_info_data.left_score as i32);
        app.set_right_score(match_info_data.right_score as i32);
        app.set_timer(match_info_data.timer as i32);
        app.set_period(match_info_data.period as i32);

        app.set_weapon(match match_info_data.weapon {
            match_info::Weapon::Unknown => 0,
            match_info::Weapon::Epee => 1,
            match_info::Weapon::Sabre => 2,
            match_info::Weapon::Fleuret => 3,
        });

        app.set_priority(match match_info_data.priority {
            match_info::Priority::Left => -1,
            match_info::Priority::None => 0,
            match_info::Priority::Right => 1,
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
}
