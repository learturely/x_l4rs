#![cfg(feature = "ehall_login_impl")]
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
