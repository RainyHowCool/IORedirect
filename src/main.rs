use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
mod redirect;
pub use redirect::redirect_io;

fn execute_message(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    if let Ok(_) = stream.read(&mut buf) {
        println!("Received Message:\n {}", String::from_utf8_lossy(&buf));
        stream.write_all(b"HTTP/1.1 200 OK\nContent-Type: text/html\n\n<code>").unwrap();
        redirect_io("/bin/sh".to_string(),stream.try_clone().unwrap());
        stream.write_all(b"</code>").expect("");
    }
}

fn main() -> std::io::Result<()>{
    let ver = "0.0.1";
    println!("IORedirect v{ver}");
    // Create server
    let svr = TcpListener::bind("127.0.0.1:54731")?;
    // listen
    for stream in svr.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");
                thread::spawn(move || execute_message(stream));
            }
            Err(e) => println!("Error in {:?}",e),
        }
    }
    Ok(())
}
