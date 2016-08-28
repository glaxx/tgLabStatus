//    Copyright (C) 2015 Stefan Luecke
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
//
extern crate telegram_bot;
extern crate hyper;
#[macro_use]
extern crate version;

use std::io::Read;
use telegram_bot::*;

const VERSION: &'static str = version!();

fn main() {
    let token = "";
    let api = Api::from_token(token).unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    let client = hyper::client::Client::new();

    let res = listener.listen(|u| {
        if let Some(m) = u.message {
            let name = m.from.first_name;

            match m.msg {
                MessageType::Text(t) => {
                    if t == "/status" {
                        let status =
                            client.get("https://labctl.openlab-augsburg.de/sphincter/?action=state")
                                .send();
                        match status {
                            Ok(mut response) => {
                                let mut buf = String::new();
                                let body = response.read_to_string(&mut buf);
                                match body {
                                    Ok(_) => {
                                        try!(api.send_message(m.chat.id(),
                                                              format!("Hey {}! The Lab status \
                                                                       is {}",
                                                                      name,
                                                                      buf),
                                                              None,
                                                              None,
                                                              None,
                                                              None));
                                    }
                                    Err(e) => {
                                        try!(api.send_message(m.chat.id(),
                                                              format!("Sorry {}! The Lab \
                                                                       status is currently \
                                                                       unavailable. Please Try \
                                                                       again later.",
                                                                      name),
                                                              None,
                                                              None,
                                                              None,
                                                              None));
                                        println!("Error: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                try!(api.send_message(m.chat.id(),
                                                      format!("Sorry {}! The Lab status is \
                                                               currently unavailable. Please \
                                                               Try again later.",
                                                              name),
                                                      None,
                                                      None,
                                                      None,
                                                      None));
                                println!("Error: {}", e);
                            }
                        }

                    }

                    if t == "/help" || t == "/start" {
                        try!(api.send_message(m.chat.id(),
                                              format!("Hello {},\nthis is \
                                                       @OpenLabAugsburgBot.\n\nRight now I \
                                                       support the following \
                                                       commands:\n\n/help - show this \
                                                       text\n/status - show the room \
                                                       status\n/version - show the bots \
                                                       version number\n\nFor any issues visit \
                                                       https://github.com/glaxx/tglabstatus",
                                                      name),
                                              None,
                                              None,
                                              None,
                                              None));
                    }

                    if t == "/version" {
                        try!(api.send_message(m.chat.id(),
                                              format!("Version: {}", VERSION),
                                              None,
                                              None,
                                              None,
                                              None));
                    }
                }

                _ => {
                    // No other MessageTypes supported right now
                }
            }
        }

        Ok(ListeningAction::Continue)
    });

    match res {
        Ok(_) => {
            println!("Programm terminated succesfully");
        }
        Err(e) => {
            println!("An error occured: {}", e);
        }
    }
}
