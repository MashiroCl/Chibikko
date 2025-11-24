use rdev::{listen, Event, EventType};
use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

pub type Counter = Arc<Mutex<HashMap<String, u64>>>;

pub fn start_keylogger(counter: Counter){
    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_name = format!("{:?}", key);

                let mut map = counter.lock().unwrap();
                *map.entry(key_name).or_insert(0) +=1;
            }
        };

        if let Err(err) = listen(callback) {
            println!("Error: {:?}", err);
        }
    });
    
}

