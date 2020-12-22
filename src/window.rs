use gtk::prelude::*;
use std::fs;

pub struct Window {
    pub widget: gtk::ApplicationWindow,//window to hold the layout
    pub label: gtk::Label,//label to hold the temp
}

impl Window {


    //call when window object is created
    pub fn start_timer(self) {

        let timerclojure = move || {
            let temp =  fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .or::<String>(Ok("0".to_string()))
            .map(|val| val.split_whitespace().collect::<String>())
            .map(|val| val.parse::<f32>().unwrap())
            .map(|val| val / 1000.0)
            .unwrap();
            self.label.set_label(format!("{:?} °C",temp).as_str());
            Continue(true)//required return type for a timer :)
        };
        gtk::timeout_add(1000,timerclojure);
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
