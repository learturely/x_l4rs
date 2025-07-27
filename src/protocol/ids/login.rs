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
use crate::{error::AgentError, protocol::ids::IDSProtocolItem, utils::percent_enc};
use log::debug;
use ureq::{Agent, Body, http::Response};

#[inline]
pub fn login_page(agent: &Agent, target: &str) -> Result<Response<Body>, AgentError> {
    let target = percent_enc(target);
    Ok(agent
        .get(&format!("{}?service={target}", IDSProtocolItem::Login))
        .call()?)
}

// #[derive(Serialize, Debug)]
// pub struct LoginPostData {
//     pub(crate) username: String,
//     pub(crate) password: String,
//     pub(crate) remember_me: bool,
//     pub(crate) captcha: String,
//     #[serde(rename = "_eventId")]
//     pub(crate) event_id: String,
//     pub(crate) cllt: String,
//     pub(crate) dllt: String,
//     pub(crate) lt: String,
//     pub(crate) execution: String,
// }
#[inline]
pub fn login(
    agent: &Agent,
    target: &str,
    data: Vec<(&str, &str)>,
) -> Result<Response<Body>, AgentError> {
    let target = percent_enc(target);
    Ok(agent
        .post(&format!("{}?service={}", IDSProtocolItem::Login, target))
        .send_form(data)?)
}
#[inline]
pub fn has_logged_in(agent: &Agent) -> bool {
    agent
        .get(IDSProtocolItem::Authserver.get())
        .config()
        .max_redirects(0)
        .build()
        .call()
        .is_ok_and(|r| {
            let code = r.status();
            debug!("{code}");
            code != 302
        })
}

#[cfg(test)]
mod tests {
    use crate::protocol::ids::has_logged_in;
    use log::info;
    use ureq::Agent;

    #[test]
    fn test_authserver() {
        let agent = Agent::new_with_config(Agent::config_builder().max_redirects(0).build());
        let r = has_logged_in(&agent);
        info!("{r}");
    }
}
