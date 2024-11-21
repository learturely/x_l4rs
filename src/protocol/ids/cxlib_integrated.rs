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

use crate::protocol::ids::IDSProtocolItem;
use cxlib_protocol::{ProtocolDataTrait, ProtocolItemTrait, ProtocolTrait};
use onceinit::OnceInit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IDSProtocolData {
    login: Option<String>,
    check_need_captcha: Option<String>,
    verify_slider_captcha: Option<String>,
    open_slider_captcha: Option<String>,
    authserver: Option<String>,
    get_user_conf: Option<String>,
}

impl ProtocolItemTrait for IDSProtocolItem {
    type ProtocolData = IDSProtocolData;

    fn config_file_name() -> &'static str {
        "x_l4rs-ids-protocol.toml"
    }

    fn get_protocol_() -> &'static OnceInit<(dyn ProtocolTrait<IDSProtocolItem> + 'static)> {
        &IDS_PROTOCOL
    }

    fn get_protocol() -> &'static dyn ProtocolTrait<Self> {
        &*IDS_PROTOCOL
    }

    fn get_default(&self) -> String {
        IDSProtocolItem::get_default(self).to_string()
    }
}

impl Default for IDSProtocolData {
    fn default() -> Self {
        IDSProtocolData {
            login: Some(ProtocolItemTrait::get_default(&IDSProtocolItem::Login)),
            check_need_captcha: Some(ProtocolItemTrait::get_default(
                &IDSProtocolItem::CheckNeedCaptcha,
            )),
            verify_slider_captcha: Some(ProtocolItemTrait::get_default(
                &IDSProtocolItem::VerifySliderCaptcha,
            )),
            open_slider_captcha: Some(ProtocolItemTrait::get_default(
                &IDSProtocolItem::OpenSliderCaptcha,
            )),
            authserver: Some(ProtocolItemTrait::get_default(&IDSProtocolItem::Authserver)),
            get_user_conf: Some(ProtocolItemTrait::get_default(
                &IDSProtocolItem::GetUserConf,
            )),
        }
    }
}

impl ProtocolDataTrait for IDSProtocolData {
    type ProtocolItem = IDSProtocolItem;

    fn map_by_enum<'a, T>(
        &'a self,
        t: &IDSProtocolItem,
        do_something: impl Fn(&'a Option<String>) -> T,
    ) -> T {
        match t {
            IDSProtocolItem::Login => do_something(&self.login),
            IDSProtocolItem::CheckNeedCaptcha => do_something(&self.check_need_captcha),
            IDSProtocolItem::VerifySliderCaptcha => do_something(&self.verify_slider_captcha),
            IDSProtocolItem::OpenSliderCaptcha => do_something(&self.open_slider_captcha),
            IDSProtocolItem::Authserver => do_something(&self.authserver),
            IDSProtocolItem::GetUserConf => do_something(&self.get_user_conf),
        }
    }
    fn map_by_enum_mut<'a, T>(
        &'a mut self,
        t: &IDSProtocolItem,
        do_something: impl Fn(&'a mut Option<String>) -> T,
    ) -> T {
        match t {
            IDSProtocolItem::Login => do_something(&mut self.login),
            IDSProtocolItem::CheckNeedCaptcha => do_something(&mut self.check_need_captcha),
            IDSProtocolItem::VerifySliderCaptcha => do_something(&mut self.verify_slider_captcha),

            IDSProtocolItem::OpenSliderCaptcha => do_something(&mut self.open_slider_captcha),
            IDSProtocolItem::Authserver => do_something(&mut self.authserver),
            IDSProtocolItem::GetUserConf => do_something(&mut self.get_user_conf),
        }
    }
}

static IDS_PROTOCOL: OnceInit<dyn ProtocolTrait<IDSProtocolItem>> = OnceInit::new();
