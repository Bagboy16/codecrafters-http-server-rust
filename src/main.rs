// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    // Create a listener bound to 
    // unwrap() is used to panic if the listener can't be created
    // panic! is a macro that prints an error message and exits the program
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // listener.incoming() returns an iterator over incoming connections
    
    for stream in listener.incoming() {
        // match is used to handle the Result returned by incoming()
        // if the Result is Ok, the stream is printed 
        // if the Result is Err, the error is printed
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
