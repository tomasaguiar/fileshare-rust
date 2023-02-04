use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // Read the file name from the client
    let mut file_name = vec![];
    let _ = stream.read_to_end(&mut file_name).unwrap();
    let file_name = String::from_utf8(file_name).unwrap();

    // Read the contents of the file into a buffer
    let buffer = fs::read(file_name).unwrap();

    // Send the contents of the file to the client
    stream.write_all(&buffer).unwrap();
}

pub fn server_connection() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
