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

pub struct VersionHandler {
}

impl VersionHandler {
    pub fn new() -> VersionHandler {
        VersionHandler {}
    }
}

impl Handler for VersionHandler {
    fn msg_type() -> telegram_bot::MessageType {
        telegram_bot::MessageType::Text("".to_string())
    }

    fn command() -> Vec<String> {
        vec![String::from_str("/version").unwrap()]
    }

    fn process(&self,
               m: telegram_bot::Message,
               a: telegram_bot::Api)
               -> Result<telegram_bot::Message, telegram_bot::Error> {
        a.send_message(m.chat.id(),
                       format!("Version: {}", version!()),
                       None,
                       None,
                       None,
                       None)
    }
}
