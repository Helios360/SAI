use std::{
    io,
    thread::{self, sleep},
    time::Duration,
    process::Command,
};

use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use rand::Rng;
//use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use fantoccini::{ClientBuilder, Locator};// activation of webdriver
//use display_info::DisplayInfo;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> { 
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // asking for users insta credentials
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
        .spawn()  // starts the process and returns immediately
        .expect("Failed to start geckodriver");
        
    // activating the webdriver with some specs
    // connecting using "native" TLS (with feature `native-tls`; on by default)
    // connection from local
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://www.instagram.com/").await?; // going to insta
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.instagram.com/");
    let _ = c.set_window_rect(0,0,600,600).await; // set window size and coords corelated with the monitor (broken)

    //set of instructions for the computer to go
    let _ = thread::sleep(Duration::from_secs(2));
    humanize(300,510);
    let _ = thread::sleep(Duration::from_secs(1));
    let _ = enigo.button(Button::Left,Click);
    let _ = thread::sleep(Duration::from_secs(2));
    let _ = enigo.key(Key::Tab,Click);
    enigo.text(&username.trim()).unwrap();
    let _ = enigo.key(Key::Tab,Click);
    enigo.text(&passwd.trim()).unwrap();
    let _ = thread::sleep(Duration::from_secs(1));
    let _ = enigo.key(Key::Return,Click);
    let _ = thread::sleep(Duration::from_secs(9));
    humanize(425,562);
    let _ = enigo.button(Button::Left,Click);
    let _ = thread::sleep(Duration::from_secs(2));
    humanize(295,480);
    let _ = enigo.button(Button::Left,Click);
    let _ = thread::sleep(Duration::from_secs(1));
    humanize(60,196);
    let _ = enigo.button(Button::Left,Click);

    let notif = c.find(Locator::Css(".x1n2onr6 div div div div div div div div span")).await?;
    let inner_notif = div.inner_html().await?;



    sleep(Duration::from_secs(1000));
    //let status = child.wait().expect("Failed to wait on child process");
    if let Err(e) = child.kill() {
        eprintln!("Failed to kill geckodriver: {}", e);
    } else {
        println!("Geckodriver process terminated");
    }
    c.close().await
}


//If the first 4 letters of the css class are not equal to html then it's a voc message, have to record


fn humanize(x: i32, y: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let (currentx, currenty) = match enigo.location() {
        Ok(pos) => pos,
        Err(e) => panic!("Failed to get cursor location: {:?}", e),
    };

    let steps = 70;
    let delay = 10;
    let mut rng = rand::thread_rng();

    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let send_x = currentx + ((x - currentx) as f32 * t) as i32;
        let send_y = currenty + ((y - currenty) as f32 * t) as i32;
        
        let offsetx: i32 = rng.gen_range(-2..3);  // Equivalent to -2..=2
        let offsety: i32 = rng.gen_range(-2..3);


        enigo.move_mouse(send_x + offsetx, send_y + offsety,Coordinate::Abs);
        thread::sleep(Duration::from_millis(delay));
    }

    enigo.move_mouse(x, y,Coordinate::Abs);
}
