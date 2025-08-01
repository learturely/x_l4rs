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

#![cfg(feature = "ids")]
use crate::protocol::ids::IDSProtocolItem;
use serde::Serialize;
use ureq::{Agent, Body, http::Response};

/// ``` json
/// {
///     cn: "我是谁"
///     en: "WhoAmI"
/// }
/// ```
#[inline]
pub fn get_user_conf(agent: &Agent) -> Result<Response<Body>, Box<ureq::Error>> {
    #[derive(Serialize)]
    struct Data {
        n: f64,
    }
    Ok(agent
        .post(IDSProtocolItem::GetUserConf.get())
        .send_json(Data {
            n: 0.12724911253015814, // 似乎没用。
        })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    #[test]
    fn test_get_user_conf() {
        let r = get_user_conf(&Agent::new_with_defaults()).unwrap();
        info!("{}", r.into_body().read_to_string().unwrap());
    }
}
