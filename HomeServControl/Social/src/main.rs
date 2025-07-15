use std::process::{Command, Child};
use std::{thread, time::Duration};
use std::env;

use enigo::{Enigo, Mouse, Settings, Coordinate, Button};
use enigo::Direction::Click;

fn start_xvfb() -> Child {
    Command::new("Xvfb")
        .arg(":99")
        .arg("-screen")
        .arg("0")
        .arg("1024x768x24")
        .spawn()
        .expect("Failed to start Xvfb")
}

fn main() {
    // 1. Start Xvfb manually
    let mut xvfb = start_xvfb();
    println!("Xvfb started on :99");

    // 2. Set DISPLAY for current process and children
    env::set_var("DISPLAY", ":99");

    // 3. Launch Node.js Playwright script
    let mut node = Command::new("node")
        .arg("Dm_Monitor/dm_monitor.js")
        .arg("https://instagram.com")
        .spawn()
        .expect("Failed to start Playwright script");

    // 4. Wait a few seconds for browser to launch
    thread::sleep(Duration::from_secs(5));

    // 5. Use Enigo with Settings
    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");

    // 6. Move mouse and click inside the virtual X screen
    enigo.move_mouse(200, 150, Coordinate::Abs).expect("Mouse move failed");
    thread::sleep(Duration::from_millis(500));
    enigo.button(Button::Right, Click).expect("Mouse click failed");

    println!("Mouse interaction performed inside xvfb.");

    // 7. Keep running or perform more actions
    thread::sleep(Duration::from_secs(10));

    // 8. Cleanup
    let _ = node.kill();
    let _ = xvfb.kill();
    println!("Processes terminated.");
}
