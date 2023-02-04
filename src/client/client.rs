use std::io::{Read, Write};
use std::net::TcpStream;

pub fn client_connection() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");

    // Send the file name to the server
    let file_name = "example.txt";
    stream.write(file_name.as_bytes()).unwrap();

    // Read the contents of the file into a buffer
    let mut buffer = vec![];
    let _ = stream.read_to_end(&mut buffer).unwrap();

    // Write the contents of the buffer to a file
    std::fs::write(file_name, buffer).unwrap();
}
