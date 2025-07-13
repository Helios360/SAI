use std::process::{Command, Stdio};
use std::{thread, time::Duration};

fn main() {
    let display_num = ":98";

    // Step 1: Start Xvfb (virtual display)
    let xvfb = Command::new("Xvfb")
        .arg(display_num)
        .arg("-screen")
        .arg("0")
        .arg("1280x1024x24")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Xvfb");

    println!("Started Xvfb on display {}", display_num);
    thread::sleep(Duration::from_secs(2)); // wait for Xvfb to initialize

    // Step 2: Launch LibreWolf using the virtual display
    let librewolf = Command::new("librewolf")
        .env("DISPLAY", display_num)
        .arg("--no-remote") // allows multiple instances
        .arg("-P")
        .arg("HISM_Main")     // use a named profile (must already exist)
        .arg("https://www.instagram.com/direct/inbox/")
        .spawn()
        .expect("Failed to launch LibreWolf");

    println!("LibreWolf launched in virtual display.");

    // Step 3: Wait or interact here (add automation if needed)
    thread::sleep(Duration::from_secs(30)); // or loop + automate

    // Cleanup: Xvfb and LibreWolf will continue unless killed
    // You could optionally kill them here (if not managed externally)
    // For now, just inform:
    println!("LibreWolf is running. Manually stop if needed.");
}
