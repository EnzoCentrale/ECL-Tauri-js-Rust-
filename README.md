<div align="center">

# ECL+ WiFi Connector

### Lightweight menu bar app for Centrale Lyon WiFi authentication

[![Tauri](https://img.shields.io/badge/Tauri-2.9.2-blue.svg)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-70.2%25-orange.svg)](https://www.rust-lang.org/)

*A side project I made because I was tired of typing my password every day*

[Download](#installation) • [How it Works](#how-it-works)

</div>

---

## What is this?

Side project for Centrale Lyon students. I got tired of manually entering the password for the school WiFi system each time I turned my computer on, so I made an app to fix it.

It's a simple menu bar application that automates the WiFi login process. Just press CMD/Windows + R on the login page and it fills everything in for you.

Principally made for MacOs.
I gotta build it on a Windows machine to get you guys a easy download link should work? 
no idea if it works for Linux i didn't go that deep into programming yet, tell me if it works.

**Security note**: Username/passwords are saved locally on your machine, no connection whatsoever to a server for security reasons.

Built with Tauri (Rust backend, JavaScript/HTML frontend).

## Installation

Download the app here:

[**ecl+_1.0.0_aarch64.dmg**](https://github.com/EnzoCentrale/ECL-WifiConnector/blob/main/ecl%2B_1.0.0_aarch64.dmg)

**Setup:**
1. Download ecl+.dmg
2. Give the app accessibility permission (needed for autofill to work)
3. Submit your credentials in the app
4. Press CMD/Windows + R on the ECL WiFi page to autoconnect

That's it.

## How it Works

### Tech Stack

The app uses Tauri, which means:
- Backend in Rust (handles all the logic, AND FASTT)
- Frontend in HTML/CSS/JavaScript (simple UI)

Also way more lightweight than electron , AND i'm never touching XCode again

### How the Request Flow Works

**When you start the program:**
1. Frontend (`main.js`) calls `listen_for_cmd_r_and_write()` with your credentials
2. Rust backend starts a global hotkey listener for CMD+R (with global hotkey directory)
4. App shows green indicator - it's now listening

**When you press CMD+R:**
1. `global-hotkey` crate detects the keypress system-wide
2. Triggers `write_pass()` function
3. `enigo` crate simulates keyboard input:
   - Presses TAB (to focus username field)
   - Types your username
   - Presses TAB (to focus password field)  
   - Types your password
   - Presses ENTER (submits the form)
4. You're logged in

**The menu bar part:**
- `lib.rs` sets up the system tray icon (i didn't understand much of that part)
- On macOS, it hides the dock icon so it only lives in the menu bar
- Click the icon to show/hide the window


## Resources Used

### Rust Dependencies

| Crate | Version | What it does |
|-------|---------|--------------|
| `tauri` | 2.9.2 | Main framework - builds the app |
| `tauri-plugin-opener` | 2 | Opens URLs |
| `tauri-plugin-notification` | 2 | Shows system notifications |
| `tauri-plugin-shell` | 2 | Runs shell commands if needed |
| `serde` / `serde_json` | 1 | JSON handling for frontend ↔ backend communication |
The two external rust plugins i acc used: 
| `enigo` | 0.6.1 | Simulates keyboard typing |
| `global-hotkey` | 0.7.0 | Listens for CMD+R anywhere |
(I was using rdev before but ITS SO SLOW, and made everthing lag)


### Frontend

Just basic HTML/CSS/JS, nothing fancy. Uses Tauri's built-in API to call Rust functions (was a pain to implement)
Frontend was designed with Framer 

## Privacy

WARNING : if someone goes on your computer and does CMD-R on a plain text document while the program is running , IT WILL Print your password.
Not my fault

## ENERGY CONSUMPTION
Iv've been running tests on my computer , over the span of 12 hours it has consumes <1% battery life , so nothing to worry about 

---

<div align="center">

Made by a lazy student who hates typing passwords

*For Centrale Lyon*

</div>
