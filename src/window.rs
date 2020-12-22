use gtk::prelude::*;
extern crate systemstat;

use systemstat::{System, Platform};


pub struct Window {
    pub widget: gtk::ApplicationWindow,//window to hold the layout
    pub label: gtk::Label,//label to hold the temp
}

impl Window {

    //call when window object is created
    pub fn start_timer(self) {

        let timerclojure = move || {
            let sys = System::new();
            match sys.cpu_temp() {//can we get the temperatures from the sys object?
                Ok(cpu_temp) => { self.label.set_label(format!("{} {}",cpu_temp.to_string().as_str()," °C").as_str());},//yes, show it!
                Err(_) =>{},//no, do nothing
            }
            Continue(true)//required return type for a timer :)
        };
        gtk::timeout_add(2000,timerclojure);
    }


    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        let label:gtk::Label = builder
            .get_object("label")
            .expect("Failed to get the label");

        Self { widget,label }
    }
}
