// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    UNKNOWN,
}
// Implement the FromStr trait for the HttpMethod enum to convert a string to an HttpMethod value
impl HttpMethod {
    fn from_str(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::UNKNOWN,
        }
    }
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
            HttpMethod::TRACE => "TRACE".to_string(),
            HttpMethod::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
}

struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: String,
    headers: Vec<String>,
}

fn parse_http_request(request: &str) -> Option<HttpRequest> {
    let parts: Vec<&str> = request.split("r\n\r\n").collect();
    let header_lines: Vec<&str> = parts.get(0)?.split("\r\n").collect();
    let req_line: Vec<&str> = header_lines.get(0)?.split_whitespace().collect();
    Some(HttpRequest {
        method: HttpMethod::from_str(req_line.get(0)?),
        path: req_line.get(1)?.to_string(),
        version: req_line.get(2)?.to_string(),
        headers: header_lines[1..].iter().map(|s| s.to_string()).collect(),
    })
}

fn handle_response(request: HttpRequest) -> String {
    let http_200_ok: &str = "HTTP/1.1 200 OK\r\n"; // create a response to send back to the client. It is a string slice of type &str (a reference to a string) that contains the response headers.
    let http_400_not_found: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    if request.path.contains("/echo/") {
        // Test 4: Respond with content for the /echo/ path
        let res_data: &str = &request.path.trim().replace("/echo/", "");
        let content_length: &str = &format!("Content-Length: {}\r\n", res_data.len());
        let content_type: &str = "Content-Type: text/plain\r\n";
        let response: String = format!(
            "{}{}{}\r\n{}",
            http_200_ok, content_type, content_length, res_data
        );
        return response;
    }
    if request.path == "/user-agent" {
        let user_agent: &str = request
            .headers
            .iter()
            .find(|s| s.contains("User-Agent: "))
            .unwrap();
        let res_data: &str = &user_agent.trim().replace("User-Agent: ", "");
        let content_length: &str = &format!("Content-Length: {}\r\n", res_data.len());
        let content_type: &str = "Content-Type: text/plain\r\n";
        let response: String = format!(
            "{}{}{}\r\n{}",
            http_200_ok, content_type, content_length, res_data
        );
        return response;
    }
    if request.path == "/" {
        // Test 2: Respond with 200 OK for the root path
        let response: String = format!("{}\r\n", http_200_ok);
        return response;
    }
    http_400_not_found.to_string() //Test 3: Respond with 404 NOT FOUND for any other path
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024]; // create a buffer to hold the incoming data, it is an array of 1024 bytes of type u8 (unsigned 8-bit integer)
    stream.read(&mut buffer).unwrap(); // read the incoming data into the buffer. &mut is used to pass a mutable reference to the buffer so that it can be modified

    let request: HttpRequest = parse_http_request(&String::from_utf8_lossy(&buffer)).unwrap(); // parse the request from the buffer. from_utf8_lossy() is used to convert the buffer to a string. It is used because the parse_http_request() function expects a string slice of type &str
    println!("Method: {}", request.method.to_string()); // print the request to the console
    println!("Path: {}", request.path);
    println!("Version: {}", request.version);

    let response: String = handle_response(request); // handle the request and get the response
    println!("Response: {}", response); // print the response to the console
    stream.write(response.as_bytes()).unwrap(); // write the response to the stream. as_bytes() is used to convert the response to a byte slice of type &[u8]
    stream.flush().unwrap(); // flush the stream to ensure that the response is sent. unwrap() is used to panic if the flush fails
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Create a listener bound to
    // unwrap() is used to panic if the listener can't be created
    // panic! is a macro that prints an error message and exits the program
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // listener.incoming() returns an iterator over incoming connections

    for stream in listener.incoming() {
        // match is used to handle the Result returned by incoming()
        // if the Result is Ok, the stream is printed
        // if the Result is Err, the error is printed
        thread::spawn(|| match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
