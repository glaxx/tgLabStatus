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

const MAX_FAILURES: u64 = 3;

fn main() {
    let api = Api::from_env("OPENLAB_AUGSBURG_BOT_TOKEN").unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    let mut failures: u64 = 0;

    loop {
        let a = Api::from_env("OPENLAB_AUGSBURG_BOT_TOKEN").unwrap();;
        let res = listener.listen(move |u| {
            if let Some(m) = u.message {
                let mclone = m.clone();
                if let MessageType::Text(t) = m.msg {
                    if handler::status::StatusHandler::command().contains(&t) {
                        let statush = handler::status::StatusHandler::new();
                        statush.process(mclone.clone(), a.clone());
                    }

                    if handler::start::StartHandler::command().contains(&t) {
                        let starth = handler::start::StartHandler::new();
                        starth.process(mclone.clone(), a.clone());
                    }

                    if handler::version::VersionHandler::command().contains(&t) {
                        let versionh = handler::version::VersionHandler::new();
                        versionh.process(mclone.clone(), a.clone());
                    }
                }
            }
            Ok(ListeningAction::Continue)
        });

        match res {
            Ok(_) => {
                println!("Program terminated succesfully");
            }
            Err(e) => {
                failures += 1;
                println!("[{}/{}]An error occured: {}", failures, MAX_FAILURES, e);
            }
        }
        if failures == MAX_FAILURES {
            return;
        }
    }
}
