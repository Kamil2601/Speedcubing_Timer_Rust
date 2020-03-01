#[macro_use]
mod clone;
mod timer;

extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate stopwatch;
use gio::prelude::*;
use gtk::prelude::*;
use std::env;

use std::rc::Rc;
use std::cell::RefCell;


fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_default_size(800, 600);
    window.set_title("Speedcubing timer");

    let time_run = false;

    let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let timer = Rc::new(RefCell::new(timer::Timer::new()));
    let content = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let cube_choice = gtk::ComboBoxText::new();
    cube_choice.append_text("2x2");
    cube_choice.append_text("3x3");
    cube_choice.append_text("4x4");
    cube_choice.set_active(Some(1));
    header_box.pack_start(&gtk::Label::new(None), true, true, 0);
    header_box.pack_start(&cube_choice, true, true, 0);
    header_box.pack_start(&gtk::Label::new(None), true, true, 0);
    content.pack_start(&header_box, false, false, 0);
    content.pack_start(&timer.borrow_mut().scramble.label, false, false, 0);
    content.pack_start(&timer.borrow_mut().label, true, true, 0);

    cube_choice.connect_changed(clone!(timer =>move |combo|{
        let active = combo.get_active();
        match active{
            Some(0) => timer.borrow_mut().cube2x2(),
            Some(1) => timer.borrow_mut().cube3x3(),
            Some(2) => timer.borrow_mut().cube4x4(),
            _ => (),
        }
    }));

    window.add(&content);
    //timer.borrow_mut().init();

    window.connect_key_press_event(clone!(timer => move |_, event| {
        if event.get_keyval() == gdk::enums::key::space {
            if !timer.borrow().stopwatch.is_running(){
                cube_choice.set_sensitive(false);
            } else{
                cube_choice.set_sensitive(true);
            }
            timer.borrow_mut().space_press();
            
        }
        Inhibit(false)
        }));

    window.connect_key_release_event(clone!(timer => move |_, event| {
        if event.get_keyval() == gdk::enums::key::space {
            timer.borrow_mut().space_release();
        }
        Inhibit(false)
        }));

    timeout_add(100,move ||{
        if timer.borrow().stopwatch.is_running(){
            timer.borrow_mut().markup_solve();
        }
        return glib::Continue(true);
    });    

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("speedcubing.timer"), Default::default())
        .expect("Application::new failed");
    application.connect_activate(&activate);
    application.run(&env::args().collect::<Vec<_>>());
}
