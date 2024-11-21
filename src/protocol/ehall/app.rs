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
use ureq::Agent;

pub fn use_app(agent: &Agent, app_id: &str) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!("{}?appId={app_id}", EhallProtocolItem::AppShow))
        .set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8",
        )
        .call()?)
}

pub fn get_app_list(agent: &Agent, search_key: &str) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!(
            "{}?{}&{}&{}&{}&{}",
            EhallProtocolItem::ServiceSearchCustom,
            format_args!("searchKey={search_key}"),
            "pageNumber=1",
            "pageSize=150",
            "sortKey=recentUseCount",
            "orderKey=desc",
        ))
        .call()?)
}
