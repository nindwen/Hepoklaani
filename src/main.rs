mod replacements;
mod transforms;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufRead, Write, BufReader, Error};
use std::{thread, str};

#[cfg(debug_assertions)]
static DOMAIN: &'static str = "localhost:8086";

#[cfg(not(debug_assertions))]
static DOMAIN: &'static str = "hepoklaani.usvs.xyz";

// Handling single connection
fn handle_client(stream: TcpStream) -> Result<(), Error> {

    let mut client_connection = BufReader::new(stream);
    let mut request = String::new();
    let mut request_body = String::new();

    // Read request headers
    for line in client_connection.by_ref().lines() {
        let current = transforms::header_mutate(line?);
        request += &current;
        request += "\r\n";
        if current == "" {
            break;
        }
    }

    let content_length = transforms::parse_content_length(&request);

    // Read request body
    // We assume it's a single line
    let _  = client_connection.by_ref()
        .take(content_length as u64)
        .read_line(&mut request_body);

    request += &request_body;

    // Connect to remote
    let mut remote_connection = TcpStream::connect("bioklaani.fi:80")?;

    // Relay the request
    remote_connection.write_all(request.as_bytes())?;
    remote_connection.flush()?;
    let _ = remote_connection.shutdown(std::net::Shutdown::Write);

    // Send remote's response to client
    let mut response = vec![0; 0];
    remote_connection.read_to_end(&mut response)?;
    let bytes = transforms::form_response(response);
    client_connection.into_inner().write_all(&bytes)?;

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8086").unwrap();

    // Launch new thread for every connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    match handle_client(stream) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Thread returned with error: {}", e); 
                        }
                    }
                });
            }
            Err(e) => { 
                println!("Error on incoming connection: {}", e); 
            }
        }
    }
}
