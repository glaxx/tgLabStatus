//    Copyright (C) 2016 Stefan Luecke
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU Affero General Public License as published
//    by the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the GNU Affero General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
//    Authors: Stefan Luecke <glaxx@glaxx.net>
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
    fn msg_type() -> telegram_bot::MessageType {
        telegram_bot::MessageType::Text("".to_string())
    }


    fn command() -> Vec<String> {
        vec![String::from_str("/status").unwrap()]
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
}
