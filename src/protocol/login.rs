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

use crate::{protocol::XL4rsProtocolItem, utils::percent_enc};
use cxlib_protocol::ProtocolItemTrait;
use log::debug;
use std::ops::Deref;
use ureq::{Agent, AgentBuilder};

pub fn login_page(agent: &Agent, target: &str) -> Result<ureq::Response, Box<ureq::Error>> {
    let target = percent_enc(target);
    Ok(agent
        .get(&format!("{}?service={target}", XL4rsProtocolItem::Login))
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
pub fn login(
    agent: &Agent,
    target: &str,
    data: &[(&str, &str)],
) -> Result<ureq::Response, Box<ureq::Error>> {
    let target = percent_enc(target);
    Ok(agent
        .post(&format!("{}?service={}", XL4rsProtocolItem::Login, target))
        .send_form(data)?)
}
pub fn is_logged_in(agent: &Agent) -> bool {
    // TODO: UserAgent.
    // TODO: use ureq 3.x.
    let agent = AgentBuilder::new()
        .redirects(0)
        .cookie_store(agent.cookie_store().deref().clone())
        .build();
    agent
        .get(&XL4rsProtocolItem::Authserver.get())
        .call()
        .is_ok_and(|r| {
            let code = r.status();
            debug!("{code}");
            code != 302
        })
}

#[cfg(test)]
mod tests {
    use crate::protocol::is_logged_in;
    use log::info;
    use ureq::AgentBuilder;

    #[test]
    fn test_authserver() {
        let agent = AgentBuilder::new().redirects(0).build();
        let r = is_logged_in(&agent);
        info!("{r}");
    }
}
