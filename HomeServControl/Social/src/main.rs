use std::process::{Command, Stdio};
use std::{thread, time::Duration};
use tiny_http::{Server, Response};

fn main() {
    let display_num = ":98";
    let mut xvfb = Command::new("Xvfb")
        .arg(display_num)
        .arg("-screen")
        .arg("0")
        .arg("1280x1024x24")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Xvfb");

    println!("Started Xvfb on display {}", display_num);
    thread::sleep(Duration::from_secs(2));
    let x11vnc_result = Command::new("x11vnc")
        .args([
            "-display", display_num,
            "-rfbport", "5998",  // Bind explicitly to match DISPLAY :98
            "-nopw",
            "-forever",
            "-shared",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();


    let mut node_script = Command::new("node")
        .arg("Dm_Monitor/dm_monitor.js")
        .env("DISPLAY", display_num)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Node script");

    println!("Node.js monitor launched.");
    
    
    let server = Server::http("127.0.0.1:3000").unwrap();
    println!("Listening on http://localhost:3000...");

    for request in server.incoming_requests() {
        println!("Request received: {:?}", request);

        println!("Method: {:?}", request.method());
        println!("URL: {:?}", request.url());

        let response = Response::from_string("OK");
        request.respond(response).unwrap();
    }

    xvfb.kill().ok();
    node_script.kill().ok();
}
