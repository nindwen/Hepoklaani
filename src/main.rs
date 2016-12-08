use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufRead, Write, BufReader, Error};
use std::{result, thread, i64, str};

#[cfg(debug_assertions)]
static DOMAIN: &'static str = "localhost:8086";

#[cfg(not(debug_assertions))]
static DOMAIN: &'static str = "hepoklaani.usvs.xyz";

fn handle_client(mut stream: TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut request = String::new();
    let mut requestBody = String::new();

    for line in reader.by_ref().lines() {
        let mut current = header_mutate(line?);
        request += &current;
        request += "\r\n";
        println!("{}", current);
        if current == "" {
            break;
        }
    }

    let mut startPos: usize = 0;
    let contentLength = request.find("Content-Length")
        .and_then(|pos| {
            startPos = pos;
            request[pos..].find("\r\n")
        })
    .and_then(|endPos| {
        request[startPos..endPos]
            .parse::<usize>().ok()
    }).unwrap_or(0);

    let requestBodyVec = reader.get_ref()
        .bytes()
        .take(contentLength)
        .map(|x| x.unwrap_or(0))
        .collect::<Vec<_>>();
    let requestBody = str::from_utf8(&requestBodyVec)
        .unwrap_or("");
    request += &requestBody;

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
        "Transfer-Encoding: chunked".to_string()
    } else {
        line.replace(DOMAIN,"bioklaani.fi")
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
            .replace("bioklaani.fi",DOMAIN)
            .replace("Bio-Klaani","Hepoklaani")
            .replace("Guardian","Hevordian")
            .replace("Pave","Hevo")
            .replace("MaKe@nurkka|_.)","HePo@nurkka|_.)")
            .replace("Kerosiinipelle","Heporillipelle")
            .replace("Igor","Hepor")
            .replace("Kapura","Hepura")
            .replace("Keetongu","Heepongu")
            .replace("Manfred","Horsfred")
            .replace("susemppu","Hevonen")
            .replace("Don","HooKoo")
            .replace("Klaanon","Hevoset the fanfic")
            .replace("Klaanilehti","Hevossanomat")
            .replace("ELKOM","SUURI HEVONEN")
            .replace("Meistä","Hevosista")
            .replace("Baten","Hevosen")
            .replace("Bate","Hevonen")
            .replace("Matoro TBS","Heporo TBS")
            .replace("Peelo","Heepo")
            .replace("img src=\"./download/file.php?avatar=" ,"img src=\"https://files.nindwen.blue/hepoklaani/hepoava.png\" alt=\"")
            .replace("/headers/","https://files.nindwen.blue/hepoklaani/hepoklaani.png");

        // Chunk information
        body += "\r\n";
        body += &format!("{:x}", newSection.len());
        body += "\r\n";
        body += &newSection;
    }
    // Terminating chunk
    body += "\r\n0\r\n\r\n";

    let mut header = headerAndBody[0]
        .replace("bioklaani.fi",DOMAIN);
    header.push_str("\r\n");
    //header.push_str(&("\r\nContent-Length: ".to_string() + &body.len().to_string() + "\r\n"));

    let response = header + &body;
    println!("Response: \n{}\n!", response);
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8086").unwrap();

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
