// To anyone reading this , i kept this because of nostalgia and the pain it caused me to use rdev
use enigo::{
    Direction::{Click},
    Enigo, Key, Keyboard, Settings,
};
use rdev::{listen, Event, EventType, Key as KK};


#[tauri::command]
pub fn write_pass(user : &str,pass : &str){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Tab,Click).unwrap();
    let _ = enigo.text(user); // the let is just to make it so theres no error cus tspmo
    enigo.key(Key::Tab,Click).unwrap();
    let _ = enigo.text(pass); 
    enigo.key(Key::Return,Click).unwrap();
}

#[tauri::command]
pub fn listen_for_cmd_r_and_write(user: &str, pass: &str) {
    use std::sync::{Arc, Mutex};

    let user = user.to_string();
    let pass = pass.to_string();

    println!("Starting key event listener. Waiting for Cmd+M key combination...");
    let meta_down = Arc::new(Mutex::new(false));
    let meta_down_cb = meta_down.clone();

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(KK::MetaLeft) | EventType::KeyPress(KK::MetaRight) => {
                println!("Meta key pressed.");
                *meta_down_cb.lock().unwrap() = true;
            }
            EventType::KeyRelease(KK::MetaLeft) | EventType::KeyRelease(KK::MetaRight) => {
                *meta_down_cb.lock().unwrap() = false;
            }
            EventType::KeyPress(KK::KeyR) => {
                if *meta_down_cb.lock().unwrap() {
                    println!("Cmd+R detected! Calling actions::write_pass...");
                    write_pass(&user, &pass);
                } else {
                    println!("M pressed without Meta.");
                }
            }
            EventType::KeyPress(key) => {
                println!("Key pressed: {:?}", key);
            }
            _ => {}
        }
    };

    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
}

