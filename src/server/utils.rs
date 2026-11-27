use std::io::{self, BufRead, Write};

use crate::server::outgoing::OutgoingResponse;

pub fn read_frame(reader: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut content_length: Option<usize> = None;

    loop {
        let mut header_line: String = String::new();

        if reader.read_line(&mut header_line)? == 0 {
            return Ok(None); // EOI
        }

        let trimmed_line: &str = header_line.trim();

        if trimmed_line.is_empty() {
            break;
        }

        if let Some((name, value)) = trimmed_line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().ok();
            }
        }
    }

    let length: usize = content_length.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length header.")
    })?;

    let mut body: Vec<u8> = vec![0; length];

    reader.read_exact(&mut body)?;

    Ok(Some(body))
}

pub fn write_frame(writer: &mut impl Write, response: &OutgoingResponse) -> io::Result<()> {
    let body: Vec<u8> = serde_json::to_vec(response).expect("response serialization cannot fail.");

    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;

    writer.write_all(&body)?;
    writer.flush()
}
