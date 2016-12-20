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
        let slice = &request[startPos+16..startPos + endPos];

        slice.parse::<usize>().ok()
    }).unwrap_or(0);


    let requestBodyVec = reader.get_ref()
        .bytes()
        .take(contentLength)
        .map(|x| x.unwrap_or(0))
        .collect::<Vec<_>>();
    let requestBody = str::from_utf8(&requestBodyVec)
        .unwrap_or("");
    println!("ContentLength: {}, body: \n{}\n---\n",contentLength,requestBody);
    request += &requestBody;

    let mut connection = TcpStream::connect("bioklaani.fi:80").unwrap();

    println!("{}",request);
    connection.write_all(request.as_bytes())?;
    connection.flush()?;
    connection.shutdown(std::net::Shutdown::Write);

    let mut response = vec![0; 0];
    connection.read_to_end(&mut response)?;

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

fn send_response(mut stream: TcpStream, resp: Vec<u8>) {
    let bytes = match str::from_utf8(&resp.clone()) {
        Ok(resp) => {
            let response = resp.to_string();

            let headerAndBody: Vec<_> = response.split("\r\n\r\n").collect();

            let mut header = headerAndBody[0]
                .replace("bioklaani.fi",DOMAIN);

            let mut body = String::new();

            if (header.contains("Content-Length")) {
                let (_, tail) = headerAndBody.split_at(1);
                body = tail.iter().fold(String::new(), |cat, x| cat + x);
                body += "\r\n";

                header.push_str(&(
                        "\r\nContent-Length: ".to_string() + &body.len().to_string())
                    .to_string());
                body = "\r\n".to_string() + &body;
            } else {
                let bodySections = headerAndBody[1]
                    .split("\r\n")
                    // Filter away the sections with chunk length
                    .filter(|section| {
                        match i64::from_str_radix(section, 16) {
                            Ok(size) => false,
                            Err(_) => true,
                        }
                    });

                for section in bodySections {
                    let newSection = contentReplacements(section.to_string());

                    // Chunk information
                    body += "\r\n";
                    body += &format!("{:x}", newSection.len());
                    body += "\r\n";
                    body += &newSection;
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

fn contentReplacements(content: String) -> String {
    content
        .replace("bioklaani.fi",DOMAIN)
        .replace("Bio-Klaani","Hepoklaani")
        .replace("Klaanon","Hevoset the fanfic")
        .replace("Klaanilehti","Hevossanomat")
        .replace("Bio-Logi","Heppapäiväkirja")
        .replace("Guardian","Shit Biscuit")
        .replace("Don","HooKoo")
        .replace("MaKe@nurkka|_.)","Hepo@talli|🐎")
        .replace("Kerosiinipelle","Heporinttipelle")
        .replace("Igor","Hegor")
        .replace("Kapura","Hepura")
        .replace("Keetongu","Heevongu")
        .replace("Manfred","Horsfred")
        .replace("susemppu","Hevonen")
        .replace("Nenya","Ponya")
        .replace("Visokki","Hepokki")
        .replace("Umbra","Dr.U")
        .replace("Dr.U","Heppatohtori")
        .replace("Snowman","Snowie")
        .replace("Snowie","Lumihevonen")
        .replace("Killjoy","Horsejoy")
        .replace("Domek the light one","Valohevonen")
        .replace("Pave","Hevo")
        .replace("Suga","Heavy Metal Poica")
        .replace("ELKOM","SUURI HEVONEN")
        .replace("Meistä","Hevosista")
        .replace("Baten","Hevosen")
        .replace("Bate","Hevonen")
        .replace("Matoro TBS","Warhistory Sparklehoof")
        .replace("Peelo","Heepo")
        .replace("img src=\"./download/file.php?avatar=" ,"img src=\"https://files.nindwen.blue/hepoklaani/hepoava.png\" alt=\"")
        .replace("/headers/","https://files.nindwen.blue/hepoklaani/hepoklaani.png")
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
