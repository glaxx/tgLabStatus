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

use std::str::FromStr;
use handler::Handler;

pub struct StartHandler {
}

impl StartHandler {
    pub fn new() -> StartHandler {
        StartHandler {}
    }
}

impl Handler for StartHandler {
    fn msg_type() -> telegram_bot::MessageType {
        telegram_bot::MessageType::Text("".to_string())
    }

    fn command() -> String {
        String::from_str("/start").unwrap()
    }

    fn process(&self, m: telegram_bot::Message) -> String {
        format!("Hello {},\nthis is @OpenLabAugsburgBot.\n\nRight now I support \
                 the following commands:\n\n/help - show \
                 this text\n/status - show the room \
                 status\n/version - show the bots version \
                 number\n\nFor any issues visit \
                 https://github.com/glaxx/tglabstatus",
                m.from.first_name)
    }
}
