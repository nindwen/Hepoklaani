use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufRead, Write, BufReader, Error};
use std::result;
use std::thread;
use std::i64;


fn handle_client(mut stream: TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut request = String::new();

    for line in reader.by_ref().lines() {
        let mut current = header_mutate(line?);
        request += &current;
        request += "\r\n";
        println!("{}", current);
        if current == "" {
            break;
        }
    }

    let mut connection = TcpStream::connect("bioklaani.fi:80").unwrap();
    //let mut connection = TcpStream::connect("localhost:8082").unwrap();
    connection.write_all(request.as_bytes())?;
    connection.flush()?;
    connection.shutdown(std::net::Shutdown::Write);

    let mut response = String::new();
    connection.read_to_string(&mut response)?;

    send_response(reader.into_inner(), response);

    Ok("miau".to_string())
}

fn header_mutate(line: String) -> String {
    if line.contains("Accept-Encoding") {
        "Accept-Encoding: chunked".to_string()
    } else if line.contains("Transfer-Encoding") {
        "Transfer-Encoding: identity".to_string()
    } else {
        line.replace("localhost:8081","bioklaani.fi")
    }
}

fn send_response(mut stream: TcpStream, response: String) {
    let headerAndBody: Vec<_> = response.split("\r\n\r\n").collect();
    let bodySections = headerAndBody[1]
        .split("\r\n")
        .filter(|section| {
            match i64::from_str_radix(section, 16) {
                Ok(size) => false,
                Err(_) => true,
            }
        });

    let mut body = String::new();
    for section in bodySections {
        let newSection = section
            .replace("bioklaani.fi","localhost:8081")
            .replace("Bio-Klaani","Hepoklaani")
            .replace("Guardian","Hevordian")
            .replace("Pave","Hepo")
            .replace("Matoro TBS","Heporo TBS")
            .replace("/headers/","https://files.nindwen.blue/hepoklaani/hepoklaani.png");

        // Chunk information
        body += "\r\n";
        body += &format!("{:x}", newSection.len());
        body += "\r\n";
        body += &newSection;
        body += "\r\n";
    }
    // Terminating chunk
    body += "0\r\n\r\n";

    let mut header = headerAndBody[0]
        .replace("bioklaani.fi","localhost:8081");
    header.push_str("\r\n");
    //header.push_str(&("\r\nContent-Length: ".to_string() + &body.len().to_string() + "\r\n"));

    let response = header + &body;
    println!("Response: \n{}\n!", response);
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => { }
        }
    }
}
