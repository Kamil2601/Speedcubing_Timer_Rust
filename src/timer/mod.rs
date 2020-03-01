
mod scramble;

use gtk::prelude::*;
use stopwatch::Stopwatch;
//use scramble::Scramble;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Timer {
    pub stopwatch: Stopwatch,
    //pub cube_choice: Rc::<RefCell<gtk::ComboBox>>,
    //pub cube_choice: gtk::ComboBox,
    pub label: gtk::Label,
    pub release_to_start: bool,
    pub scramble: scramble::Scramble,
}

impl Timer {
    pub fn new() -> Self {
        let mut result = Timer {
            stopwatch: Stopwatch::new(),
            //cube_choice: Rc::new(RefCell::new(gtk::ComboBox::new())),
            //cube_choice: gtk::ComboBox::new(),
            label: gtk::Label::new(Some("0.00")),
            release_to_start: false,
            scramble: scramble::Scramble::new3x3(),
        };

        result.init();
        
        return result;
    }
    pub fn init(&mut self) {
        let markup = "<span font_desc='Ubuntu 150'>0.00</span>";
        self.label.set_markup(&markup);
        self.scramble.generate();
        self.scramble.show();
    }

    pub fn cube2x2(&mut self){
        self.scramble.to2x2();
        self.init();
    }

    pub fn cube3x3(&mut self){
        self.scramble.to3x3();
        self.init();
    }
    
    pub fn cube4x4(&mut self){
        self.scramble.to4x4();
        self.init();
    }

    pub fn space_press(&mut self) {
        if !self.stopwatch.is_running() {
            self.release_to_start = true;
            self.markup_ready();
        } else {
            self.release_to_start = false;
            self.stopwatch.stop();
            self.markup_end();
            self.scramble.reset();
        }
    }

    pub fn space_release(&mut self) {
        if self.release_to_start {
            self.stopwatch.reset();
            self.stopwatch.start();
            self.markup_solve();
        }
    }

    pub fn markup_ready(&mut self) {
        self.label
            .set_markup("<span foreground='green' font_desc='Ubuntu 150'>0.0</span>");
    }

    pub fn markup_solve(&mut self) {
        let mut time = self.format_time();
        time.pop();
        self.label
            .set_markup(format!("<span font_desc='Ubuntu 150'>{}</span>", time).as_str());
    }

    pub fn format_time(&self) -> String {
        let time = self.stopwatch.elapsed();
        let sec_part = time.as_millis() / 10 % 100;
        let seconds = time.as_secs() % 60;
        let mins = time.as_secs() / 60;
        let time: String;
        if mins > 0 {
            time = format!("{}:{}.{:02}", mins, seconds, sec_part);
        } else {
            time = format!("{}.{:02}", seconds, sec_part);
        }

        return time;
    }

    pub fn markup_end(&mut self) {
        let time = self.format_time();
        self.label
            .set_markup(format!("<span font_desc='Ubuntu 150'>{}</span>", time).as_str());
    }
}
