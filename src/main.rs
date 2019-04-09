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

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        fn serde_to_ws(err: serde_json::error::Error) -> Result<Request> {
            Err(ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), ""))
        };

        match &msg {
            Message::Text(text) => {
                let r = serde_json::from_str(&text).or_else(&serde_to_ws)?;
                match &r {
                    Request::Ping { message: _ } => self.out.send("pong"),
                    Request::Pong { number } => {
                        for _ in 1..(*number) {
                            self.out.send("ding dong")?;
                        }
                        self.out.send("ding dong")
                    }
                }
            }
            _ => self.out.send(
                serde_json::to_string(&Response::Error {
                    reason: "expected text, got binary".to_owned(),
                })
                .or_else(|err| Err(ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), "")))?,
            ),
        }
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
