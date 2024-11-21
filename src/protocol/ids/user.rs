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

#![cfg(feature = "ids")]
use crate::protocol::ids::IDSProtocolItem;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;
use serde::Serialize;
use ureq::Agent;
/// ``` json
/// {
///     cn: "我是谁"
///     en: "WhoAmI"
/// }
/// ```
pub fn get_user_conf(agent: &Agent) -> Result<ureq::Response, Box<ureq::Error>> {
    #[derive(Serialize)]
    struct Data {
        n: f64,
    }
    Ok(agent
        .post(IDSProtocolItem::GetUserConf.get().as_str())
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
        let r = get_user_conf(&Agent::new()).unwrap();
        info!("{}", r.into_string().unwrap());
    }
}
