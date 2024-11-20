#![cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::{ProtocolDataTrait, ProtocolItemTrait, ProtocolTrait};
use onceinit::OnceInit;
use serde::{Deserialize, Serialize};
use crate::protocol::XL4rsProtocolItem;


#[derive(Serialize, Deserialize)]
pub struct XL4rsProtocolData {
    login: Option<String>,
    check_need_captcha: Option<String>,
    verify_slider_captcha: Option<String>,
    open_slider_captcha: Option<String>,
    authserver: Option<String>,
    get_user_conf: Option<String>,
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
        XL4rsProtocolItem::get_default(self).to_string()
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
