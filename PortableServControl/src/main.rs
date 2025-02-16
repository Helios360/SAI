mod gui;
use gui::App;
use iced::{Settings, Application};
use tokio::time::{sleep, Duration};
use tokio::task;
use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() -> iced::Result {
    println!("Start");
    let (tx, rx) = mpsc::channel();
    
    // Send initial username
    tx.send(String::from("Hellios")).unwrap();

    // Spawn another example task
    task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("End of simulated operation");
    });

    // Create and run the GUI application with the receiver
    let gui_settings = Settings::default();
    
    // Start the sync thread before running the app
    thread::spawn(move || {
        gui::sync(rx);
    });

    // Run the GUI application
    App::run(gui_settings)
}