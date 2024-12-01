//     [xdlinux/libxduauth] for Rust.
//     Copyright (C) 2024  learturely <learturely@gmail.com>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod cry;
mod find_element;

pub use cry::*;
pub(crate) use find_element::*;

use ureq::{Agent, AgentBuilder};

pub fn get_now_timestamp_mills() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("系统时间异常。")
        .as_millis()
}

pub fn build_agent() -> Agent {
    let cookie_store = cookie_store::CookieStore::new(None);
    AgentBuilder::new()
        .redirects(15)
        .cookie_store(cookie_store)
        .build()
}
pub fn build_agent_with_user_agent(ua: &str) -> Agent {
    let cookie_store = cookie_store::CookieStore::new(None);
    AgentBuilder::new()
        .redirects(15)
        .user_agent(ua)
        .cookie_store(cookie_store)
        .build()
}

pub fn time_it<R, F: FnOnce() -> R>(f: F) -> (R, u128) {
    let start = std::time::Instant::now();
    let r = f();
    let elapsed = start.elapsed();
    (r, elapsed.as_millis())
}

pub fn print_timed_result<T>(result: (T, u128)) -> T {
    println!("{}", result.1);
    result.0
}
