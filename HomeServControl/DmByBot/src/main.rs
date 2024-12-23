use std::io;
use std::{thread,time};
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;
use fantoccini::ClientBuilder;// activation of webdriver
use rdev::{listen, Event} /*activate when needing mouse coords*/;use rdev::{simulate, Button, EventType, Key, SimulateError};//inputs for the bot
use display_info::DisplayInfo;
//use std::time::Instant;
//test
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> { 
    
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
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");// connection from local
    c.goto("https://www.instagram.com/").await?;//going to insta
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.instagram.com/");
    //c.set_window_rect(0,0,1000,1000).await;// set window size and coords corelated with the monitor (broken)
    
    
    
    /*----------------------------ATTENTION!!!----------------------------/

    The commented part bellow has been abandoned because it is not "usefull" for the project,
    the following part had the purpose to scan monitors to detects the main monitor to be able to sync mouse coords and window coords
    but the first purpose of SAI is to run on a minimalistic environment so no triple screen or things like that,
    the program will only work on the SAI environement machine only screen...
    
    // getting the position of the main windows(where the selenium instance will start) DO NOT USE !
    //let start = Instant::now();println!("{}",start);
    let mut main_posx=0;
    let mut main_posy=0;
    let display_infos = DisplayInfo::all().unwrap();
    for display_info in display_infos {
        println!("{display_info:?}",);
        //println!("{}",display_info.width);
        if display_info.is_primary == true{
            break;
        }else if display_info.is_primary == false{
            main_posx+= display_info.width;
            main_posy+= display_info.height;
        }
        println!("{}",main_posx);
    }

    */


    //click(640,280,main_posx,main_posy);// click makes the mouse go to desired coord with adapted screen size and nth
    //click(640,1160/*,main_posx,main_posy*/);DEV MODE (wqhd screen at left of the main screen "me :3") so; mainScreenHeight - leftScreenHeight + mouseHeightCoord = mouseHeightClick
    click(640,800/*,main_posx,main_posy*/);
    thread::sleep(time::Duration::from_secs(2));
    //click(840,707);
    click(832,300);
    match type_text(&username) {
        Ok(_) => println!("Password typed successfully!"),
        Err(err) => eprintln!("Error typing password: {:?}", err),
    }
    send(&EventType::KeyPress(Key::Tab));
    match type_text(&passwd) {
        Ok(_) => println!("Password typed successfully!"),
        Err(err) => eprintln!("Error typing password: {:?}", err),
    }
    thread::sleep(time::Duration::from_secs(2));
    click(846,800);
    thread::sleep(time::Duration::from_secs(2));
    click(45,444);


    //can use this to print coords on cli when you make inputs
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
    
    fn callback(event: Event) {
        println!("My callback {:?}", event);
        match event.name {
            Some(string) => println!("User wrote {:?}", string),
            None => (),
        }
    }



    sleep(Duration::from_secs(10));
    //let status = child.wait().expect("Failed to wait on child process");
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
    // let the OS catchup (at least MacOS eww)
    thread::sleep(delay);
}
//,main_posx:u32,main_posy:u32(to add in params)
fn click(posx:u32,posy_minus_height:u32){
    //let xed=main_posx as f64 + posx as f64;comments are part of the more than one screen not used feature
    //let yed=main_posy as f64 - posy_minus_height as f64;
    //send(&EventType::MouseMove { x: xed, y: yed });
    send(&EventType::MouseMove { x: posx as f64, y: posy_minus_height as f64});
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}

fn type_text(input: &str) -> Result<(), SimulateError> {
    for c in input.chars() {
        if let Some((key, shift)) = char_to_key(c) {
            if shift {
                simulate(&EventType::KeyPress(Key::ShiftLeft))?;
            }
            simulate(&EventType::KeyPress(key))?;
            simulate(&EventType::KeyRelease(key))?;
            if shift {
                simulate(&EventType::KeyRelease(Key::ShiftLeft))?;
            }
        } else {
        type_altgr_symbol(c).unwrap();
    }
        sleep(Duration::from_millis(50));
    } 
    Ok(())
}

// yes I asked chatgpt for the lists whatsup
fn char_to_key(c: char) -> Option<(Key, bool)> {
    match c {
        // Lowercase letters
        'a' => Some((Key::KeyA, false)),
        'b' => Some((Key::KeyB, false)),
        'c' => Some((Key::KeyC, false)),
        'd' => Some((Key::KeyD, false)),
        'e' => Some((Key::KeyE, false)),
        'f' => Some((Key::KeyF, false)),
        'g' => Some((Key::KeyG, false)),
        'h' => Some((Key::KeyH, false)),
        'i' => Some((Key::KeyI, false)),
        'j' => Some((Key::KeyJ, false)),
        'k' => Some((Key::KeyK, false)),
        'l' => Some((Key::KeyL, false)),
        'm' => Some((Key::KeyM, false)),
        'n' => Some((Key::KeyN, false)),
        'o' => Some((Key::KeyO, false)),
        'p' => Some((Key::KeyP, false)),
        'q' => Some((Key::KeyQ, false)),
        'r' => Some((Key::KeyR, false)),
        's' => Some((Key::KeyS, false)),
        't' => Some((Key::KeyT, false)),
        'u' => Some((Key::KeyU, false)),
        'v' => Some((Key::KeyV, false)),
        'w' => Some((Key::KeyW, false)),
        'x' => Some((Key::KeyX, false)),
        'y' => Some((Key::KeyY, false)),
        'z' => Some((Key::KeyZ, false)),

        // uppercase letters (Shift required)
        'A' => Some((Key::KeyA, true)),
        'B' => Some((Key::KeyB, true)),
        'C' => Some((Key::KeyC, true)),
        'D' => Some((Key::KeyD, true)),
        'E' => Some((Key::KeyE, true)),
        'F' => Some((Key::KeyF, true)),
        'G' => Some((Key::KeyG, true)),
        'H' => Some((Key::KeyH, true)),
        'I' => Some((Key::KeyI, true)),
        'J' => Some((Key::KeyJ, true)),
        'K' => Some((Key::KeyK, true)),
        'L' => Some((Key::KeyL, true)),
        'M' => Some((Key::KeyM, true)),
        'N' => Some((Key::KeyN, true)),
        'O' => Some((Key::KeyO, true)),
        'P' => Some((Key::KeyP, true)),
        'Q' => Some((Key::KeyQ, true)),
        'R' => Some((Key::KeyR, true)),
        'S' => Some((Key::KeyS, true)),
        'T' => Some((Key::KeyT, true)),
        'U' => Some((Key::KeyU, true)),
        'V' => Some((Key::KeyV, true)),
        'W' => Some((Key::KeyW, true)),
        'X' => Some((Key::KeyX, true)),
        'Y' => Some((Key::KeyY, true)),
        'Z' => Some((Key::KeyZ, true)),

        // Numbers (Shift required)
        '1' => Some((Key::Num1, true)), // & 
        '2' => Some((Key::Num2, true)), // é
        '3' => Some((Key::Num3, true)), // "
        '4' => Some((Key::Num4, true)), // '
        '5' => Some((Key::Num5, true)), // (
        '6' => Some((Key::Num6, true)), // -
        '7' => Some((Key::Num7, true)), // è
        '8' => Some((Key::Num8, true)), // _
        '9' => Some((Key::Num9, true)), // ç
        '0' => Some((Key::Num0, true)), // à

        // Symbols
        '&' => Some((Key::Num1, false)),
        'é' => Some((Key::Num2, false)),
        '"' => Some((Key::Num3, false)),
        '\'' => Some((Key::Num4, false)),
        '(' => Some((Key::Num5, false)),
        '-' => Some((Key::Num6, false)),
        'è' => Some((Key::Num7, false)),
        '_' => Some((Key::Num8, false)),
        'ç' => Some((Key::Num9, false)),
        'à' => Some((Key::Num0, false)),    
        ')' => Some((Key::Minus, false)),
        '=' => Some((Key::Equal, false)),

        // Common punctuation
        '.' => Some((Key::Dot, true)),
        ',' => Some((Key::SemiColon, false)),
        ';' => Some((Key::Dot, false)),
        ':' => Some((Key::Slash, true)),
        '!' => Some((Key::Num8, true)),

        '#' | '@' | '|' | '~' | '€' => None,

        _ => None,
    }
}
fn type_altgr_symbol(c: char) -> Result<(), SimulateError> {
    match c {
        '#' => {
            simulate(&EventType::KeyPress(Key::AltGr))?;
            simulate(&EventType::KeyPress(Key::Num3))?;
            simulate(&EventType::KeyRelease(Key::Num3))?;
            simulate(&EventType::KeyRelease(Key::AltGr))?;
        }
        '@' => {
            simulate(&EventType::KeyPress(Key::AltGr))?;
            simulate(&EventType::KeyPress(Key::Num0))?;
            simulate(&EventType::KeyRelease(Key::Num0))?;
            simulate(&EventType::KeyRelease(Key::AltGr))?;
        }
        '|' => {
            simulate(&EventType::KeyPress(Key::AltGr))?;
            simulate(&EventType::KeyPress(Key::Num6))?;
            simulate(&EventType::KeyRelease(Key::Num6))?;
            simulate(&EventType::KeyRelease(Key::AltGr))?;
        }
        '~' => {
            simulate(&EventType::KeyPress(Key::AltGr))?;
            simulate(&EventType::KeyPress(Key::Num2))?;
            simulate(&EventType::KeyRelease(Key::Num2))?;
            simulate(&EventType::KeyRelease(Key::AltGr))?;
        }
        '€' => {
            simulate(&EventType::KeyPress(Key::AltGr))?;
            simulate(&EventType::KeyPress(Key::KeyE))?;
            simulate(&EventType::KeyRelease(Key::KeyE))?;
            simulate(&EventType::KeyRelease(Key::AltGr))?;
        }
        _ => {}
    }
    Ok(())
}

/*For QWERTY keyboards !!!
fn char_to_key(c: char) -> Option<(Key, bool)> {
    match c {
        // lowercase letters
        'a' => Some((Key::KeyA, false)),
        'b' => Some((Key::KeyB, false)),
        'c' => Some((Key::KeyC, false)),
        'd' => Some((Key::KeyD, false)),
        'e' => Some((Key::KeyE, false)),
        'f' => Some((Key::KeyF, false)),
        'g' => Some((Key::KeyG, false)),
        'h' => Some((Key::KeyH, false)),
        'i' => Some((Key::KeyI, false)),
        'j' => Some((Key::KeyJ, false)),
        'k' => Some((Key::KeyK, false)),
        'l' => Some((Key::KeyL, false)),
        'm' => Some((Key::KeyM, false)),
        'n' => Some((Key::KeyN, false)),
        'o' => Some((Key::KeyO, false)),
        'p' => Some((Key::KeyP, false)),
        'q' => Some((Key::KeyQ, false)),
        'r' => Some((Key::KeyR, false)),
        's' => Some((Key::KeyS, false)),
        't' => Some((Key::KeyT, false)),
        'u' => Some((Key::KeyU, false)),
        'v' => Some((Key::KeyV, false)),
        'w' => Some((Key::KeyW, false)),
        'x' => Some((Key::KeyX, false)),
        'y' => Some((Key::KeyY, false)),
        'z' => Some((Key::KeyZ, false)),

        // uppercase letters (Shift required)
        'A' => Some((Key::KeyA, true)),
        'B' => Some((Key::KeyB, true)),
        'C' => Some((Key::KeyC, true)),
        'D' => Some((Key::KeyD, true)),
        'E' => Some((Key::KeyE, true)),
        'F' => Some((Key::KeyF, true)),
        'G' => Some((Key::KeyG, true)),
        'H' => Some((Key::KeyH, true)),
        'I' => Some((Key::KeyI, true)),
        'J' => Some((Key::KeyJ, true)),
        'K' => Some((Key::KeyK, true)),
        'L' => Some((Key::KeyL, true)),
        'M' => Some((Key::KeyM, true)),
        'N' => Some((Key::KeyN, true)),
        'O' => Some((Key::KeyO, true)),
        'P' => Some((Key::KeyP, true)),
        'Q' => Some((Key::KeyQ, true)),
        'R' => Some((Key::KeyR, true)),
        'S' => Some((Key::KeyS, true)),
        'T' => Some((Key::KeyT, true)),
        'U' => Some((Key::KeyU, true)),
        'V' => Some((Key::KeyV, true)),
        'W' => Some((Key::KeyW, true)),
        'X' => Some((Key::KeyX, true)),
        'Y' => Some((Key::KeyY, true)),
        'Z' => Some((Key::KeyZ, true)),

        // numbers
        '0' => Some((Key::Num0, false)),
        '1' => Some((Key::Num1, false)),
        '2' => Some((Key::Num2, false)),
        '3' => Some((Key::Num3, false)),
        '4' => Some((Key::Num4, false)),
        '5' => Some((Key::Num5, false)),
        '6' => Some((Key::Num6, false)),
        '7' => Some((Key::Num7, false)),
        '8' => Some((Key::Num8, false)),
        '9' => Some((Key::Num9, false)),

        // symbols requiring Shift
        '!' => Some((Key::Num1, true)),
        '@' => Some((Key::Num2, true)),
        '#' => Some((Key::Num3, true)),
        '$' => Some((Key::Num4, true)),
        '%' => Some((Key::Num5, true)),
        '^' => Some((Key::Num6, true)),
        '&' => Some((Key::Num7, true)),
        '*' => Some((Key::Num8, true)),
        '(' => Some((Key::Num9, true)),
        ')' => Some((Key::Num0, true)),

        // common symbols
        '-' => Some((Key::Minus, false)),
        '_' => Some((Key::Minus, true)),
        '=' => Some((Key::Equal, false)),
        '+' => Some((Key::Equal, true)),
        '[' => Some((Key::LeftBracket, false)),
        '{' => Some((Key::LeftBracket, true)),
        ']' => Some((Key::RightBracket, false)),
        '}' => Some((Key::RightBracket, true)),
        ';' => Some((Key::SemiColon, false)),
        ':' => Some((Key::SemiColon, true)),
        '\'' => Some((Key::Quote, false)),
        '"' => Some((Key::Quote, true)),
        ',' => Some((Key::Comma, false)),
        '<' => Some((Key::Comma, true)),
        '.' => Some((Key::Dot, false)),
        '>' => Some((Key::Dot, true)),
        '/' => Some((Key::Slash, false)),
        '?' => Some((Key::Slash, true)),
        '\\' => Some((Key::BackSlash, false)),
        '|' => Some((Key::BackSlash, true)),

        // space
        ' ' => Some((Key::Space, false)),

        _ => None, // return None for unsupported characters
    }
}
*/