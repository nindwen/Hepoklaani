use super::replacements;
use super::DOMAIN;
use std::{i64, str};

// This transforms the headers from client
// to the ones we will send to the remote
pub fn header_mutate(line: String) -> String {
    if line.contains("Accept-Encoding") {
        "Accept-Encoding: chunked".to_string()
    } else if line.contains("Transfer-Encoding") {
        "Transfer-Encoding: chunked".to_string()
    } else {
        line.replace(DOMAIN, "bioklaani.fi")
    }
}

pub fn parse_content_length(request: &String) -> usize {
    let mut start_pos: usize = 0;
    request.find("Content-Length")
        .and_then(|pos| {
            start_pos = pos;
            request[pos..].find("\r\n")
        })
        .and_then(|end_pos| {
            let slice = &request[start_pos + 16..start_pos + end_pos];

            slice.parse::<usize>().ok()
        })
        .unwrap_or(0)
}

// This transforms the response from remote,
// to the one we will send to the client
pub fn form_response(resp: Vec<u8>) -> Vec<u8> {
    // If response if valid utf8, handle it
    // Otherwise return it as-is (e.g images)
    match str::from_utf8(&resp.clone()) {
        Ok(response) => {
            let header_and_body: Vec<_> = response.split("\r\n\r\n")
                .collect();

            let mut header = header_and_body[0].replace("bioklaani.fi", DOMAIN);

            let mut body;
            let (_, tail) = header_and_body.split_at(1);
            let raw_body = replacements::content_replace(tail.join(""));

            // bioklaani.fi serves usually with
            // Content-Encoding: Chunked,
            // but wordpress-subdomains insist on Identity.
            // Identity = plain, requires Content-Length
            if header.contains("Content-Length") {
                body = raw_body;
                body += "\r\n";

                // After replacements, Content-Length may
                // be incorrect. Luckily we can simply append
                // the correct bytecount to headers.
                header.push_str(&("\r\nContent-Length: ".to_string() + &body.len().to_string())
                    .to_string());
                body = "\r\n".to_string() + &body;

            } else {
                // Chunked
                body = chunked_encode(raw_body);
            }

            header.push_str("\r\n");
            let response = header + &body;
            response.into_bytes()
        }
        Err(_) => resp,
    }
}

fn chunked_encode(body: String) -> String {
    let mut encoded = String::new();
    let body_sections = body
        .split("\r\n")
        // Filter away the sections with chunk length
        // We will calculate them ourselves
        .filter(|section| {
            match i64::from_str_radix(section, 16) {
                Ok(_) => false,
                Err(_) => true,
            }
        });

    for section in body_sections {
        let new_section = section.to_string();

        // Chunk information
        encoded += "\r\n";
        encoded += &format!("{:x}", new_section.len());
        encoded += "\r\n";
        encoded += &new_section;
    }
    // Terminating chunk
    encoded += "\r\n0\r\n\r\n";

    encoded
}
