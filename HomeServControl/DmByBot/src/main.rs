use std::io;
use std::{thread,time};
use std::time::Duration;
use std::thread::sleep;
use fantoccini::{Client,ClientBuilder};//activation of webdriver
use rdev::{listen, Event};use rdev::{simulate, Button, EventType, Key, SimulateError};//inputs for the bot
use winit::{event_loop::EventLoop,platform::desktop::EventLoopExtDesktop};//get the working window env

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> { 
    //asking for users insta credentials
    println!("Pweaaaase enter your instagram username or email :3 !!!");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("The program couldn't read the line properly sowwwyyyyyy :(");
    println!("Enter your passwod owo");
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd).expect("The program couldn't read the line properly sowwwyyyyyy :(");
    
    //activating the webdriver with some specs
    // Connecting using "native" TLS (with feature `native-tls`; on by default)
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");//connection from local
    c.goto("https://www.instagram.com/").await?;//going to insta
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.instagram.com/");
    
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
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
fn window_rected(){
    let event_loop = EventLoop::new();
    let monitors: Vec<_> = event_loop.available_monitors().collect();
    if let Some(target_monitor) = get.monitor(1){
        let screen_position=target_monitor.position();
        let screen_size=target_monitor.size();
        c.set_window_rect(screen_position.x,screen_position.y,950, 950);//set window size and coords coorelated with the monitor used with witin
    }
}
