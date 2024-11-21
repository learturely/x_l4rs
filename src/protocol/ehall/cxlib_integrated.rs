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
use cxlib_protocol::{ProtocolDataTrait, ProtocolItemTrait, ProtocolTrait};
use onceinit::OnceInit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EhallProtocolData {
    user_favorite_apps: Option<String>,
    app_show: Option<String>,
    service_search_custom: Option<String>,
}

impl ProtocolItemTrait for EhallProtocolItem {
    type ProtocolData = EhallProtocolData;

    fn config_file_name() -> &'static str {
        "x_l4rs-ehall-protocol.toml"
    }

    fn get_protocol_() -> &'static OnceInit<(dyn ProtocolTrait<EhallProtocolItem> + 'static)> {
        &EHALL_PROTOCOL
    }

    fn get_protocol() -> &'static dyn ProtocolTrait<Self> {
        &*EHALL_PROTOCOL
    }

    fn get_default(&self) -> String {
        EhallProtocolItem::get_default(self).to_string()
    }
}

impl Default for EhallProtocolData {
    fn default() -> Self {
        EhallProtocolData {
            user_favorite_apps: Some(ProtocolItemTrait::get_default(
                &EhallProtocolItem::UserFavoriteApps,
            )),
            app_show: Some(ProtocolItemTrait::get_default(&EhallProtocolItem::AppShow)),
            service_search_custom: Some(ProtocolItemTrait::get_default(
                &EhallProtocolItem::ServiceSearchCustom,
            )),
        }
    }
}

impl ProtocolDataTrait for EhallProtocolData {
    type ProtocolItem = EhallProtocolItem;

    fn map_by_enum<'a, T>(
        &'a self,
        t: &EhallProtocolItem,
        do_something: impl Fn(&'a Option<String>) -> T,
    ) -> T {
        match t {
            EhallProtocolItem::UserFavoriteApps => do_something(&self.user_favorite_apps),
            EhallProtocolItem::AppShow => do_something(&self.app_show),
            EhallProtocolItem::ServiceSearchCustom => do_something(&self.service_search_custom),
        }
    }
    fn map_by_enum_mut<'a, T>(
        &'a mut self,
        t: &EhallProtocolItem,
        do_something: impl Fn(&'a mut Option<String>) -> T,
    ) -> T {
        match t {
            EhallProtocolItem::UserFavoriteApps => do_something(&mut self.user_favorite_apps),
            EhallProtocolItem::AppShow => do_something(&mut self.app_show),
            EhallProtocolItem::ServiceSearchCustom => do_something(&mut self.service_search_custom),
        }
    }
}

static EHALL_PROTOCOL: OnceInit<dyn ProtocolTrait<EhallProtocolItem>> = OnceInit::new();
