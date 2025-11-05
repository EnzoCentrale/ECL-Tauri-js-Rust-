use enigo::{
    Direction::Click,
    Enigo, Key, Keyboard, Settings,
};
use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

static HOTKEY_MANAGER: OnceLock<Arc<Mutex<Option<(GlobalHotKeyManager, HotKey)>>>> = OnceLock::new();
static CURRENT_CREDENTIALS: OnceLock<Arc<Mutex<(String, String)>>> = OnceLock::new();

fn get_manager() -> &'static Arc<Mutex<Option<(GlobalHotKeyManager, HotKey)>>> {
    HOTKEY_MANAGER.get_or_init(|| Arc::new(Mutex::new(None)))
}

fn get_credentials() -> &'static Arc<Mutex<(String, String)>> {
    CURRENT_CREDENTIALS.get_or_init(|| Arc::new(Mutex::new((String::new(), String::new()))))
}

#[tauri::command]
pub fn write_pass(user: &str, pass: &str){
    println!("write_pass called with user: {}", user);
    
    match Enigo::new(&Settings::default()) {
        Ok(mut enigo) => {
            let _ = enigo.key(Key::Tab, Click);
            std::thread::sleep(Duration::from_millis(50));
            let _ = enigo.text(user);
            let _ = enigo.key(Key::Tab, Click);
            std::thread::sleep(Duration::from_millis(50));
            let _ = enigo.text(pass);
            let _ = enigo.key(Key::Return, Click);
            println!("Credentials typed successfully");
        }
        Err(e) => {
            eprintln!("Failed to create Enigo instance: {:?}", e);
        }
    }
}

#[tauri::command]
pub fn stop_listening() {
    println!("Stopping listener...");
    let manager_lock = get_manager();
    if let Ok(mut manager_opt) = manager_lock.lock() {
        if let Some((manager, hotkey)) = manager_opt.take() {
            match manager.unregister_all(&[hotkey]) {
                Ok(_) => println!("Hotkey unregistered."),
                Err(e) => eprintln!("Failed to unregister hotkey: {:?}", e),
            }
        }
    }
}

#[tauri::command]
pub fn listen_for_cmd_r_and_write(user: String, pass: String) {
    println!("Setting up hotkey for user: {}", user);
    
    // Update credentials
    if let Ok(mut creds) = get_credentials().lock() {
        *creds = (user.clone(), pass.clone());
    }
    
    let manager_lock = get_manager();
    let mut manager_opt = manager_lock.lock().unwrap();
    
    // If already registered, just update credentials
    if manager_opt.is_some() {
        println!("Hotkey already registered, credentials updated.");
        return;
    }
    
    // First time - register hotkey
    match GlobalHotKeyManager::new() {
        Ok(manager) => {
            // Create Cmd+R hotkey (Super = Cmd on macOS)
            let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyR);
            
            match manager.register(hotkey) {
                Ok(_) => {
                    println!("Hotkey Cmd+R registered successfully!");
                    
                    // Spawn listener thread for hotkey events
                    std::thread::spawn(move || {
                        let receiver = GlobalHotKeyEvent::receiver();
                        println!("Listening for Cmd+R...");
                        
                        loop {
                            if let Ok(event) = receiver.recv() {
                                println!("Hotkey triggered! Event: {:?}", event);
                                
                                // Get credentials and trigger autofill
                                if let Ok(creds) = get_credentials().lock() {
                                    let user = creds.0.clone();
                                    let pass = creds.1.clone();
                                    
                                    if !user.is_empty() && !pass.is_empty() {
                                        std::thread::spawn(move || {
                                            write_pass(&user, &pass);
                                        });
                                    }
                                }
                            }
                        }
                    });
                    
                    *manager_opt = Some((manager, hotkey));
                }
                Err(e) => {
                    eprintln!("Failed to register hotkey: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create hotkey manager: {:?}", e);
        }
    }
}