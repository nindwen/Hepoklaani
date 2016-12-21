mod replacements;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufRead, Write, BufReader, Error};
use std::{thread, i64, str};

#[cfg(debug_assertions)]
static DOMAIN: &'static str = "localhost:8086";

#[cfg(not(debug_assertions))]
static DOMAIN: &'static str = "hepoklaani.usvs.xyz";

// Handling single connection
fn handle_client(stream: TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut request = String::new();
    let mut request_body = String::new();

    // Read request headers
    for line in reader.by_ref().lines() {
        let current = header_mutate(line?);
        request += &current;
        request += "\r\n";
        if current == "" {
            break;
        }
    }

    // Parse Content-Length from request
    let mut start_pos: usize = 0;
    let content_length = request.find("Content-Length")
        .and_then(|pos| {
            start_pos = pos;
            request[pos..].find("\r\n")
        })
    .and_then(|end_pos| {
        let slice = &request[start_pos+16..start_pos + end_pos];

        slice.parse::<usize>().ok()
    }).unwrap_or(0);

    // Read request body
    // (We assume it's a single line)
    reader.by_ref()
        .take(content_length as u64)
        .read_line(&mut request_body).unwrap();

    request += &request_body;

    // Connect to remote
    let mut connection = TcpStream::connect("bioklaani.fi:80").unwrap();

    // Relay the request
    connection.write_all(request.as_bytes())?;
    connection.flush()?;
    let _ = connection.shutdown(std::net::Shutdown::Write);

    // Send remote's response to client
    let mut response = vec![0; 0];
    connection.read_to_end(&mut response)?;
    send_response(reader.into_inner(), response);

    // Meow :3
    Ok("miau".to_string())
}

// Client -> hepo header
fn header_mutate(line: String) -> String {
    if line.contains("Accept-Encoding") {
        "Accept-Encoding: chunked".to_string()
    } else if line.contains("Transfer-Encoding") {
        "Transfer-Encoding: chunked".to_string()
    } else {
        line.replace(DOMAIN,"bioklaani.fi")
    }
}

fn send_response(mut stream: TcpStream, resp: Vec<u8>) {
    // If response if valid utf8, handle it
    // Otherwise return it as-is (e.g images)
    let bytes = match str::from_utf8(&resp.clone()) {
        Ok(resp) => {
            let response = resp.to_string();
            let header_and_body: Vec<_> = response.split("\r\n\r\n").collect();

            let mut header = header_and_body[0]
                .replace("bioklaani.fi",DOMAIN);

            let mut body = String::new();
            let (_, tail) = header_and_body.split_at(1);
            let raw_body = replacements::content_replace (
                tail.iter().fold(String::new(), |cat, x| cat + x)
                );

            // bioklaani.fi serves usually with 
            // Content-Encoding: Chunked,
            // but wordpress-subdomains insist on Identity.
            // Identity = plain, requires Content-Length
            if header.contains("Content-Length") {
                body = raw_body;
                body += "\r\n";

                header.push_str(&(
                        "\r\nContent-Length: ".to_string() 
                        + &body.len().to_string()
                        ).to_string());
                body = "\r\n".to_string() + &body;

            } else { // Chunked
                let body_sections = raw_body
                    .split("\r\n")
                    // Filter away the sections with chunk length
                    .filter(|section| {
                        match i64::from_str_radix(section, 16) {
                            Ok(_) => false,
                            Err(_) => true,
                        }
                    });

                for section in body_sections {
                    let new_section = section.to_string();

                    // Chunk information
                    body += "\r\n";
                    body += &format!("{:x}", new_section.len());
                    body += "\r\n";
                    body += &new_section;
                }
                // Terminating chunk
                body += "\r\n0\r\n\r\n";
            }

            header.push_str("\r\n");
            let response = header + &body;
            response.into_bytes()
        }
        Err(_) => resp,
    };
    stream.write_all(&bytes).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8086").unwrap();

    // Launch new thread for every connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let _ = handle_client(stream);
                });
            }
            Err(_) => { }
        }
    }
}
