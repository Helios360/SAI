use std::io;
use std::{thread,time};
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;
use fantoccini::{Client,ClientBuilder};//activation of webdriver
use rdev::{listen, Event};use rdev::{simulate, Button, EventType, Key, SimulateError};//inputs for the bot
use execute::Execute;


#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> { 
    
    //asking for users insta credentials
    println!("Pweaaaase enter your instagram username or email :3 !!!");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("The program couldn't read the line properly sowwwyyyyyy :(");
    println!("Enter your passwod owo");
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd).expect("The program couldn't read the line properly sowwwyyyyyy :(");


    let exe_path = "C:/Programing/Programs/Projects/geckodriver";
    println!("Starting geckodriver");
    let mut child = Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(exe_path)
        .spawn()  // Starts the process and returns immediately
        .expect("Failed to start geckodriver");
        

    //activating the webdriver with some specs
    // Connecting using "native" TLS (with feature `native-tls`; on by default)
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");//connection from local
    c.goto("https://www.instagram.com/").await?;//going to insta
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.instagram.com/");
    

    c.set_window_rect(50,50,950, 950);//set window size and coords corelated with the monitor
    send(&EventType::MouseMove { x: 640.0, y: 800.0 });
    send(&EventType::ButtonPress(Button::Left));
    /*
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
    
    fn callback(event: Event) {
        println!("My callback {:?}", event);
        match event.name {
            Some(string) => println!("User wrote {:?}", string),
            None => (),
        }
    }*/
    sleep(Duration::from_secs(10));
    let status = child.wait().expect("Failed to wait on child process");
    if let Err(e) = child.kill() {
        eprintln!("Failed to kill geckodriver: {}", e);
    } else {
        println!("Geckodriver process terminated");
    }
    c.close().await
    
}
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS eww)
    thread::sleep(delay);
}