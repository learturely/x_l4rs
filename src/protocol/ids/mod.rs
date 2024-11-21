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

#[cfg(feature = "cxlib_protocol_integrated")]
mod cxlib_integrated;
#[cfg(feature = "cxlib_protocol_integrated")]
pub use cxlib_integrated::*;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;
mod captcha;
mod login;
mod user;

pub use captcha::*;
pub use login::*;
pub use user::*;

use std::fmt::Display;

pub enum IDSProtocolItem {
    Login,
    CheckNeedCaptcha,
    VerifySliderCaptcha,
    OpenSliderCaptcha,
    Authserver,
    GetUserConf,
}
impl IDSProtocolItem {
    fn get_default(&self) -> &'static str {
        match self {
            IDSProtocolItem::Login => Self::LOGIN,
            IDSProtocolItem::CheckNeedCaptcha => Self::CHECK_NEED_CAPTCHA,
            IDSProtocolItem::VerifySliderCaptcha => Self::VERIFY_SLIDER_CAPTCHA,
            IDSProtocolItem::OpenSliderCaptcha => Self::OPEN_SLIDER_CAPTCHA,
            IDSProtocolItem::Authserver => Self::AUTHSERVER,
            IDSProtocolItem::GetUserConf => Self::GET_USER_CONF,
        }
    }
}
#[cfg(not(feature = "cxlib_protocol_integrated"))]
impl IDSProtocolItem {
    pub fn get(&self) -> &'static str {
        self.get_default()
    }
}
impl IDSProtocolItem {
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
impl Display for IDSProtocolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
