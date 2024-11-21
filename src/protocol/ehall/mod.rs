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

#![cfg(feature = "ehall")]
mod app;
mod login;

pub use app::*;
pub use login::*;

#[cfg(feature = "cxlib_protocol_integrated")]
mod cxlib_integrated;
#[cfg(feature = "cxlib_protocol_integrated")]
pub use cxlib_integrated::*;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;

use std::fmt::Display;

pub enum EhallProtocolItem {
    UserFavoriteApps,
    AppShow,
    ServiceSearchCustom,
}
impl EhallProtocolItem {
    fn get_default(&self) -> &'static str {
        match self {
            EhallProtocolItem::UserFavoriteApps => Self::USER_FAVORITE_APPS,
            EhallProtocolItem::AppShow => Self::APP_SHOW,
            EhallProtocolItem::ServiceSearchCustom => Self::SERVICE_SEARCH_CUSTOM,
        }
    }
}
#[cfg(not(feature = "cxlib_protocol_integrated"))]
impl EhallProtocolItem {
    pub fn get(&self) -> &'static str {
        self.get_default()
    }
}
impl EhallProtocolItem {
    pub const USER_FAVORITE_APPS: &'static str =
        "http://ehall.xidian.edu.cn/jsonp/userFavoriteApps.json";
    pub const APP_SHOW: &'static str = "http://ehall.xidian.edu.cn//appShow";
    pub const SERVICE_SEARCH_CUSTOM: &'static str =
        "http://ehall.xidian.edu.cn/jsonp/serviceSearchCustom.json";
}
impl Display for EhallProtocolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
