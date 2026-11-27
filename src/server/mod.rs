mod incoming;
mod outgoing;
mod server;
mod utils;

use crate::server::{
    incoming::IncomingMessage,
    server::Server,
    utils::{read_frame, write_frame},
};

use serde_json::Value;
use std::{
    io::{self, BufReader},
    path::PathBuf,
    process,
};

use crate::server::outgoing::{OutgoingResponse, ResponseError};

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const SERVER_NOT_INITIALIZED: i64 = -32002;

fn try_parsing_message(
    body: &[u8],
    standard_output: &mut io::StdoutLock,
) -> io::Result<Option<IncomingMessage>> {
    Ok(match serde_json::from_slice(body) {
        Ok(message) => message,
        Err(parse_error) => {
            let error: ResponseError = ResponseError::new(PARSE_ERROR, parse_error.to_string());

            write_frame(
                standard_output,
                &OutgoingResponse::failure(Value::Null, error),
            )?;

            None
        }
    })
}

fn do_server(path: PathBuf) -> io::Result<i32> {
    let mut standard_input: BufReader<io::StdinLock> = BufReader::new(io::stdin().lock());
    let mut standard_output: io::StdoutLock = io::stdout().lock();
    let mut server: Server = Server::new();

    while let Some(body) = read_frame(&mut standard_input)? {
        let message: IncomingMessage = match try_parsing_message(&body, &mut standard_output)? {
            Some(message) => message,
            None => continue,
        };

        if message.protocol_version != "2.0" {
            let error: ResponseError =
                ResponseError::new(INVALID_REQUEST, "\"jsonrpc\" must be \"2.0\".");

            let identifier: Value = message.identifier.unwrap_or(Value::Null);

            write_frame(
                &mut standard_output,
                &OutgoingResponse::failure(identifier, error),
            )?;

            continue;
        }

        match message.identifier {
            Some(identifier) => {
                let response: OutgoingResponse =
                    match server.handle_request(&message.method, message.parameters) {
                        Ok(result) => OutgoingResponse::success(identifier, result),
                        Err(error) => OutgoingResponse::failure(identifier, error),
                    };

                write_frame(&mut standard_output, &response)?;
            }
            None => {
                if let Some(exit_code) =
                    server.handle_notification(&message.method, message.parameters)
                {
                    return Ok(exit_code);
                }
            }
        }
    }

    Ok(0)
}

pub fn server(path: PathBuf) -> ! {
    let exit_code: i32 = match do_server(path) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("Sylo server crashed: {error}");

            1
        }
    };

    process::exit(exit_code);
}
