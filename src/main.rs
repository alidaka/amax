extern crate ws;

use serde::{Deserialize, Serialize};
use ws::{listen, CloseCode, Handler, Message, Result, Sender};

#[derive(Serialize, Deserialize)]
struct RawRequest {
    data: (String, serde_json::Value),
}

/*
#[derive(Serialize, Deserialize)]
struct Request {}

#[derive(Serialize, Deserialize)]
struct Response {}
*/

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        fn serde_to_ws(err: serde_json::error::Error) -> Result<RawRequest> {
            Err(ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), ""))
        };

        match &msg {
            Message::Text(text) => {
                let r = serde_json::from_str(&text).or_else(&serde_to_ws)?;
                match r.data.0.as_ref() {
                    "ping" => self.out.send("pong"),
                    _ => self.out.send(msg),
                }
            }
            _ => self.out.send(msg),
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    listen("0.0.0.0:3000", |out| Server { out: out }).unwrap();

    println!("We're up");
}
