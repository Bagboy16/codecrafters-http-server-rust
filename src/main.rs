// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [u8; 1024] = [0; 1024]; // create a buffer to hold the incoming data, it is an array of 1024 bytes of type u8 (unsigned 8-bit integer)
    stream.read(&mut buffer).unwrap(); // read the incoming data into the buffer. &mut is used to pass a mutable reference to the buffer so that it can be modified
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // print the incoming data as a string. from_utf8_lossy() is used to convert the buffer to a string. It is used to handle invalid UTF-8 sequences. &buffer[..] is used to pass a slice of the buffer to from_utf8_lossy(). The .. operator is used to create a slice that includes all elements of the buffer.
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n"; // create a response to send back to the client. It is a string slice of type &str (a reference to a string)
    stream.write(response.as_bytes()).unwrap(); // write the response to the stream. as_bytes() is used to convert the response to a byte array. It is used because the write() method expects a byte array. unwrap() is used to panic if the write fails
    stream.flush().unwrap(); // flush the stream to ensure that the response is sent. unwrap() is used to panic if the flush fails
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    // Create a listener bound to 
    // unwrap() is used to panic if the listener can't be created
    // panic! is a macro that prints an error message and exits the program
    let listener:TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // listener.incoming() returns an iterator over incoming connections
    
    for stream in listener.incoming() {
        // match is used to handle the Result returned by incoming()
        // if the Result is Ok, the stream is printed 
        // if the Result is Err, the error is printed
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
