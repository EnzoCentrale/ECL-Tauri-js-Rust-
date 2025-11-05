use enigo::{
    Direction::Click,
    Enigo, Key, Keyboard, Settings,
};
use rdev::{listen, Event, EventType, Key as KK};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};

lazy_static::lazy_static! {
    static ref CREDENTIALS_SENDER: Mutex<Option<Sender<(String, String)>>> = Mutex::new(None);
}

#[tauri::command]
pub fn write_pass(user: &str, pass: &str){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Tab, Click).unwrap();
    let _ = enigo.text(user);
    enigo.key(Key::Tab, Click).unwrap();
    let _ = enigo.text(pass); 
    enigo.key(Key::Return, Click).unwrap();
}

#[tauri::command]
pub fn listen_for_cmd_r_and_write(user: &str, pass: &str) {
    let mut sender_lock = CREDENTIALS_SENDER.lock().unwrap();
    
    // If sender exists, just update credentials
    if let Some(sender) = sender_lock.as_ref() {
        println!("Updating credentials...");
        let _ = sender.send((user.to_string(), pass.to_string()));
        return;
    }
    
    // First time - create the listener thread
    println!("Starting listener thread for the first time...");
    
    let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    
    // Send initial credentials
    let _ = tx.send((user.to_string(), pass.to_string()));
    
    // Save sender for future updates
    *sender_lock = Some(tx);
    drop(sender_lock); // Release lock
    
    // Spawn the persistent listener thread
    std::thread::spawn(move || {
        let meta_down = Arc::new(Mutex::new(false));
        let current_creds = Arc::new(Mutex::new((String::new(), String::new())));
        
        let meta_down_cb = meta_down.clone();
        let creds_cb = current_creds.clone();
        
        // Thread to receive credential updates
        let creds_update = current_creds.clone();
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok((user, pass)) => {
                        println!("Credentials updated: user={}", user);
                        *creds_update.lock().unwrap() = (user, pass);
                    }
                    Err(_) => {
                        println!("Credential channel closed");
                        break;
                    }
                }
            }
        });
        
        println!("Listener ready. Press Cmd+R to trigger.");
        
        let callback = move |event: Event| {
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
                        let creds = creds_cb.lock().unwrap();
                        write_pass(&creds.0, &creds.1);
                    }
                }
                _ => {}
            }
        };
        
        if let Err(error) = listen(callback) {
            eprintln!("Listener error: {:?}", error);
        }
    });
}