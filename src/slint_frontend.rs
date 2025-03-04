use slint::{Timer, TimerMode};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use crate::match_info;
use crate::modules;

use crate::layouts::*;

pub struct SlintFrontend {
    tx: mpsc::Sender<modules::Message>,
    rx: mpsc::Receiver<modules::Message>,

    match_info: Arc<Mutex<match_info::MatchInfo>>,
}

fn update_data(match_info: &Arc<Mutex<match_info::MatchInfo>>, app: &Virtuoso) {
    let match_info_data = match_info.lock().unwrap();
    app.set_left_score(match_info_data.left_score as i32);
    app.set_right_score(match_info_data.right_score as i32);
    app.set_timer(match_info_data.timer as i32);
    app.set_period(match_info_data.period as i32);
    app.set_left_color_led_on(match_info_data.left_red_led_on);
    app.set_left_white_led_on(match_info_data.left_white_led_on);
    app.set_right_color_led_on(match_info_data.right_green_led_on);
    app.set_right_white_led_on(match_info_data.right_white_led_on);
}

impl SlintFrontend {
    pub const MODULE_TYPE: modules::Modules = modules::Modules::SlintFrontend;

    pub fn new(
        tx: mpsc::Sender<modules::Message>,
        rx: mpsc::Receiver<modules::Message>,
        match_info: Arc<Mutex<match_info::MatchInfo>>,
    ) -> Self {
        Self { tx, rx, match_info }
    }

    pub fn run(&mut self) {
        let app = Virtuoso::new().unwrap();

        app.set_layout(LAYOUT_1920X480);

        update_data(&self.match_info, &app);

        let weak_app = app.as_weak();
        let timer = Timer::default();

        let match_info_clone = self.match_info.clone();

        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(100),
            move || {
                if let Some(app) = weak_app.upgrade() {
                    update_data(&match_info_clone, &app);
                }
            },
        );

        app.run().unwrap();
    }
}
