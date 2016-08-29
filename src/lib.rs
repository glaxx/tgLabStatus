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
<<<<<<< 875ae71bf5d1d4d51b7bcd063d4bf3539609e73a
#[macro_use]
extern crate version;
=======
>>>>>>> [WIP] Draft of generic snd msg fn

pub mod handler;

use std::result::Result;

pub fn send(m: telegram_bot::Message,
            a: telegram_bot::Api)
            -> Result<telegram_bot::Message, telegram_bot::Error> {

    if let telegram_bot::MessageType::Text(t) = m.msg {
        return a.send_message(m.chat.id(), t, None, None, None, None);
    } else {
        Err(telegram_bot::Error::Api("Not implemented".to_string()))
    }
}
