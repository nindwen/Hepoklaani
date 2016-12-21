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
        line.replace(DOMAIN,"bioklaani.fi")
    }
}

// This transforms the response from remote,
// to the one we will send to the client
pub fn form_response(resp: Vec<u8>) -> Vec<u8> {
    // If response if valid utf8, handle it
    // Otherwise return it as-is (e.g images)
    match str::from_utf8(&resp.clone()) {
        Ok(response) => {
            let header_and_body: Vec<_> = response
                .split("\r\n\r\n")
                .collect();

            let mut header = header_and_body[0]
                .replace("bioklaani.fi",DOMAIN);

            let mut body = String::new();
            let (_, tail) = header_and_body.split_at(1);
            let raw_body = replacements::content_replace(
                tail.join("\r\n")
                );

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
                header.push_str(&(
                        "\r\nContent-Length: ".to_string() 
                        + &body.len().to_string()
                        ).to_string());
                body = "\r\n".to_string() + &body;

            } else { // Chunked
                let body_sections = raw_body
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
    }
}
