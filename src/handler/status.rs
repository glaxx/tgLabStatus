extern crate telegram_bot;
extern crate hyper;

use std::io::Read;
use std::str::FromStr;
use handler::Handler;

pub struct StatusHandler {
}

impl StatusHandler {
    pub fn new() -> StatusHandler {
        StatusHandler {}
    }
}

impl Handler for StatusHandler {
    fn msg_type(&self) -> telegram_bot::MessageType {
        telegram_bot::MessageType::Text("".to_string())
    }

    fn process(&self, m: telegram_bot::Message) -> String {
        let client = hyper::client::Client::new();
        let status = client.get("https://labctl.openlab-augsburg.de/sphincter/?action=state")
            .send();
        let name = m.from.first_name;
        let mut answer = String::new();
        match status {
            Ok(mut response) => {

                let mut buf = String::new();
                let body = response.read_to_string(&mut buf);
                match body {
                    Ok(_) => {
                        answer = format!("Hey {}! The Lab status is {}", name, buf);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        answer = format!("Sorry {}! The Lab status is currently \
                                                          unavailable. Please Try again later.",
                                         name);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                answer = format!("Sorry {}! The Lab status is currently unavailable. Please Try \
                                  again later.",
                                 name);



            }
        }
        answer
    }

    fn command(&self) -> String {
        String::from_str("/status").unwrap()
    }
}
