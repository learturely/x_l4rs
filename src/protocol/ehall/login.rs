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

use crate::protocol::ehall::EhallProtocolItem;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;
use serde::Deserialize;
use ureq::Agent;

pub fn has_logged_in(agent: &Agent) -> bool {
    agent
        .get(EhallProtocolItem::UserFavoriteApps.get().as_ref())
        .call()
        .is_ok_and(|r| {
            #[derive(Deserialize)]
            struct Tmp {
                #[serde(rename = "hasLogin")]
                has_login: bool,
            }
            let Tmp { has_login } = r.into_json().expect("failed to deserialize hasLogin");
            has_login
        })
}
