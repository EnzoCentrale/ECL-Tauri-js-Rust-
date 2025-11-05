use enigo::{
    Direction::Click,
    Enigo, Key, Keyboard, Settings,
};
use rdev::{listen, Event, EventType, Key as KK};
use std::sync::{Arc, Mutex};
use lazy_static;


lazy_static::lazy_static! {
    static ref LISTENER_RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[tauri::command]
pub fn write_pass(user : &str, pass : &str){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Tab, Click).unwrap();
    let _ = enigo.text(user);
    enigo.key(Key::Tab, Click).unwrap();
    let _ = enigo.text(pass); 
    enigo.key(Key::Return, Click).unwrap();
}

#[tauri::command]
pub fn stop_listener() {
    println!("Stopping listener...");
    *LISTENER_RUNNING.lock().unwrap() = false;
}

#[tauri::command]
pub fn listen_for_cmd_r_and_write(user: &str, pass: &str) {
    // Stop any existing listener
    stop_listener();
    
    let user = user.to_string();
    let pass = pass.to_string();
    
    std::thread::spawn(move || {
        // Mark as running
        *LISTENER_RUNNING.lock().unwrap() = true;
        
        let meta_down = Arc::new(Mutex::new(false));
        let meta_down_cb = meta_down.clone();
        let running = LISTENER_RUNNING.clone();

        println!("New listener started. Press Cmd+R to trigger.");

        let callback = move |event: Event| {
            // Check if we should stop
            if !*running.lock().unwrap() {
                println!("Listener stopped.");
                return;
            }
            
            match event.event_type {
                EventType::KeyPress(KK::MetaLeft) | EventType::KeyPress(KK::MetaRight) => {
                    *meta_down_cb.lock().unwrap() = true;
                }
                EventType::KeyRelease(KK::MetaLeft) | EventType::KeyRelease(KK::MetaRight) => {
                    *meta_down_cb.lock().unwrap() = false;
                }
                EventType::KeyPress(KK::KeyR) => {
                    if *meta_down_cb.lock().unwrap() {
                        println!("Cmd+R detected!");
                        write_pass(&user, &pass);
                    }
                }
                _ => {}
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("Listener error: {:?}", error);
        }
    });
    
    println!("Listener thread spawned.");
}