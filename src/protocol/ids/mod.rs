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
    #[inline]
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
impl IDSProtocolItem {
    #[inline]
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
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
