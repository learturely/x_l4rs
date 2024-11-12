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

mod captcha;
mod login;
mod user;

pub use captcha::*;
pub use login::*;
pub use user::*;

use cxsign_protocol::{ProtocolDataTrait, ProtocolItemTrait, ProtocolTrait};
use onceinit::OnceInit;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub enum XL4rsProtocolItem {
    Login,
    CheckNeedCaptcha,
    VerifySliderCaptcha,
    OpenSliderCaptcha,
    Authserver,
    GetUserConf,
}

#[derive(Serialize, Deserialize)]
pub struct XL4rsProtocolData {
    login: Option<String>,
    check_need_captcha: Option<String>,
    verify_slider_captcha: Option<String>,
    open_slider_captcha: Option<String>,
    authserver: Option<String>,
    get_user_conf: Option<String>,
}
impl XL4rsProtocolItem {
    pub const LOGIN: &'static str = "http://ids.xidian.edu.cn/authserver/login";
    pub const CHECK_NEED_CAPTCHA: &'static str =
        "https://ids.xidian.edu.cn/authserver/checkNeedCaptcha.htl";
    pub const VERIFY_SLIDER_CAPTCHA: &'static str =
        "https://ids.xidian.edu.cn/authserver/common/verifySliderCaptcha.htl";
    pub const OPEN_SLIDER_CAPTCHA: &'static str =
        "https://ids.xidian.edu.cn/authserver/common/openSliderCaptcha.htl";
    pub const AUTHSERVER: &'static str = "http://ids.xidian.edu.cn/authserver/index.do";

    pub const GET_USER_CONF: &'static str =
        "https://ids.xidian.edu.cn/personalInfo/common/getUserConf";
}
impl Display for XL4rsProtocolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
impl ProtocolItemTrait for XL4rsProtocolItem {
    type ProtocolData = XL4rsProtocolData;

    fn config_file_name() -> &'static str {
        "x_l4rs-protocol.toml"
    }

    fn get_protocol_() -> &'static OnceInit<(dyn ProtocolTrait<XL4rsProtocolItem> + 'static)> {
        &X_L4RS_PROTOCOL
    }

    fn get_protocol() -> &'static dyn ProtocolTrait<Self> {
        &*X_L4RS_PROTOCOL
    }

    fn get_default(&self) -> String {
        match self {
            XL4rsProtocolItem::Login => Self::LOGIN.to_string(),
            XL4rsProtocolItem::CheckNeedCaptcha => Self::CHECK_NEED_CAPTCHA.to_string(),
            XL4rsProtocolItem::VerifySliderCaptcha => Self::VERIFY_SLIDER_CAPTCHA.to_string(),
            XL4rsProtocolItem::OpenSliderCaptcha => Self::OPEN_SLIDER_CAPTCHA.to_string(),
            XL4rsProtocolItem::Authserver => Self::AUTHSERVER.to_string(),
            XL4rsProtocolItem::GetUserConf => Self::GET_USER_CONF.to_string(),
        }
    }
}

impl Default for XL4rsProtocolData {
    fn default() -> Self {
        XL4rsProtocolData {
            login: Some(XL4rsProtocolItem::LOGIN.to_string()),
            check_need_captcha: Some(XL4rsProtocolItem::CHECK_NEED_CAPTCHA.to_string()),
            verify_slider_captcha: Some(XL4rsProtocolItem::VERIFY_SLIDER_CAPTCHA.to_string()),
            open_slider_captcha: Some(XL4rsProtocolItem::OPEN_SLIDER_CAPTCHA.to_string()),
            authserver: Some(XL4rsProtocolItem::AUTHSERVER.to_string()),
            get_user_conf: Some(XL4rsProtocolItem::GET_USER_CONF.to_string()),
        }
    }
}

impl ProtocolDataTrait for XL4rsProtocolData {
    type ProtocolItem = XL4rsProtocolItem;

    fn map_by_enum<'a, T>(
        &'a self,
        t: &XL4rsProtocolItem,
        do_something: impl Fn(&'a Option<String>) -> T,
    ) -> T {
        match t {
            XL4rsProtocolItem::Login => do_something(&self.login),
            XL4rsProtocolItem::CheckNeedCaptcha => do_something(&self.check_need_captcha),
            XL4rsProtocolItem::VerifySliderCaptcha => do_something(&self.verify_slider_captcha),
            XL4rsProtocolItem::OpenSliderCaptcha => do_something(&self.open_slider_captcha),
            XL4rsProtocolItem::Authserver => do_something(&self.authserver),
            XL4rsProtocolItem::GetUserConf => do_something(&self.get_user_conf),
        }
    }
    fn map_by_enum_mut<'a, T>(
        &'a mut self,
        t: &XL4rsProtocolItem,
        do_something: impl Fn(&'a mut Option<String>) -> T,
    ) -> T {
        match t {
            XL4rsProtocolItem::Login => do_something(&mut self.login),
            XL4rsProtocolItem::CheckNeedCaptcha => do_something(&mut self.check_need_captcha),
            XL4rsProtocolItem::VerifySliderCaptcha => {
                do_something(&mut self.verify_slider_captcha)
            }

            XL4rsProtocolItem::OpenSliderCaptcha => do_something(&mut self.open_slider_captcha),
            XL4rsProtocolItem::Authserver => do_something(&mut self.authserver),
            XL4rsProtocolItem::GetUserConf => do_something(&mut self.get_user_conf),
        }
    }
}
static X_L4RS_PROTOCOL: OnceInit<dyn ProtocolTrait<XL4rsProtocolItem>> = OnceInit::new();
