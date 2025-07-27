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

mod app;
mod login;

pub use app::*;
pub use login::*;

use std::fmt::Display;

pub enum EhallProtocolItem {
    UserFavoriteApps,
    AppShow,
    ServiceSearchCustom,
}
impl EhallProtocolItem {
    #[inline]
    fn get_default(&self) -> &'static str {
        match self {
            EhallProtocolItem::UserFavoriteApps => Self::USER_FAVORITE_APPS,
            EhallProtocolItem::AppShow => Self::APP_SHOW,
            EhallProtocolItem::ServiceSearchCustom => Self::SERVICE_SEARCH_CUSTOM,
        }
    }
}

impl EhallProtocolItem {
    #[inline]
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
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
