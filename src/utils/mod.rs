// MIT License
//
// Copyright (c) 2025 2025  learturely <learturely@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod cry;
mod find_element;
mod imageproc;

pub use cry::*;
pub(crate) use find_element::*;
pub(crate) use imageproc::*;

use ureq::Agent;

#[inline]
pub fn get_now_timestamp_mills() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("系统时间异常。")
        .as_millis()
}

#[inline]
pub fn build_agent() -> Agent {
    let config = Agent::config_builder().max_redirects(15).build();
    Agent::new_with_config(config)
}
#[inline]
pub fn build_agent_with_user_agent(ua: &str) -> Agent {
    let config = Agent::config_builder()
        .max_redirects(15)
        .user_agent(ua)
        .build();
    Agent::new_with_config(config)
}

#[inline]
pub fn time_it_and_print_result<R, F: FnOnce() -> R>(f: F) -> R {
    #[cfg(debug_assertions)]
    {
        print_timed_result(time_it(f))
    }
    #[cfg(not(debug_assertions))]
    {
        f()
    }
}
#[cfg(debug_assertions)]
#[inline]
fn time_it<R, F: FnOnce() -> R>(f: F) -> (R, u128) {
    let start = std::time::Instant::now();
    let r = f();
    let elapsed = start.elapsed();
    (r, elapsed.as_millis())
}

#[cfg(debug_assertions)]
#[inline]
fn print_timed_result<T>(result: (T, u128)) -> T {
    println!("{}", result.1);
    result.0
}
