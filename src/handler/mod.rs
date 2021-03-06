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

pub mod status;
pub mod version;
pub mod start;

pub trait Handler {
    fn msg_type() -> telegram_bot::MessageType;
    fn command() -> Vec<String>;
    fn process(&self,
               m: telegram_bot::Message,
               a: telegram_bot::Api)
               -> Result<telegram_bot::Message, telegram_bot::Error>;
}
