extern crate ws;

use serde::{Deserialize, Serialize};
use ws::{listen, CloseCode, Handler, Message, Result, Sender};

#[derive(Deserialize)]
#[serde(untagged)]
enum Request {
    Ping { message: String },
    Pong { number: i32 },
}

#[derive(Serialize)]
#[serde(untagged)]
enum Response {
    Error { reason: String },
    Success { message: String },
}

struct Server {
    out: Sender,
}

fn handle_request(request: Request) -> Response {
    match request {
        Request::Ping { message: _ } => Response::Success {
            message: "pong".to_owned(),
        },
        Request::Pong { number } => {
            let mut text = "ding dong\n".to_owned();
            for _ in 1..(number) {
                text.push_str("ding dong\n");
            }
            Response::Success { message: text }
        }
    }
}

fn serde_to_ws<T>(err: serde_json::error::Error) -> Result<T> {
    Err(ws::Error::new(
        ws::ErrorKind::Custom(Box::new(err)),
        "JSON encoding/decoding error",
    ))
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let response = match &msg {
            Message::Text(text) => {
                let request = serde_json::from_str(&text).or_else(&serde_to_ws)?;
                handle_request(request)
            }
            _ => Response::Error {
                reason: "expected text, got binary".to_owned(),
            },
        };

        self.out
            .send(serde_json::to_string(&response).or_else(&serde_to_ws)?)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Status => println!("The client has left with no status code."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    listen("0.0.0.0:3000", |out| Server { out: out }).unwrap();

    println!("We're up");
}
