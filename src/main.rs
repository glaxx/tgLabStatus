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
//
extern crate telegram_bot;
extern crate hyper;
#[macro_use]
extern crate version;

mod handler;
use telegram_bot::*;
use handler::Handler;


fn main() {
    let api = Api::from_env("OPENLAB_AUGSBURG_BOT_TOKEN").unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    let statush = handler::status::StatusHandler::new();
    let versionh = handler::version::VersionHandler::new();
    let starth = handler::start::StartHandler::new();

    let res = listener.listen(move |u| {
        if let Some(m) = u.message {
            let mclone = m.clone();
            if let MessageType::Text(t) = m.msg {
                if t == statush.command() {
                    try!(api.send_message(m.chat.id(),
                                          statush.process(mclone.clone()),
                                          None,
                                          None,
                                          None,
                                          None));
                }

                    try!(api.send_message(m.chat.id(),
                    hand.process(mclone),
                    None,
                    None,
                    None,
                    None));
                }

                if t == starth.command() {
                    try!(api.send_message(m.chat.id(),
                                          starth.process(mclone.clone()),
                                          None,
                                          None,
                                          None,
                                          None));
                }

                if t == versionh.command() {
                    try!(api.send_message(m.chat.id(),
                                          versionh.process(mclone.clone()),
                                          None,
                                          None,
                                          None,
                                          None));
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
