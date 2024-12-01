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

use crate::{IDSLoginImpl, XL4rsSessionTrait};
use cxlib_error::Error;
use getset::Getters;
use image::DynamicImage;
use serde::Deserialize;
use std::ops::Deref;
use ureq::{serde_json, Agent};

pub struct EhallLoginImpl {
    inner: IDSLoginImpl,
}

impl EhallLoginImpl {
    pub fn new() -> Self {
        EhallLoginImpl {
            inner: IDSLoginImpl::TARGET_EHALL,
        }
    }
}
impl From<IDSLoginImpl> for EhallLoginImpl {
    fn from(value: IDSLoginImpl) -> Self {
        Self { inner: value }
    }
}
impl Default for EhallLoginImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for EhallLoginImpl {
    type Target = IDSLoginImpl;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EhallSession {
    agent: Agent,
}
impl Deref for EhallSession {
    type Target = Agent;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
// TODO: non_exhaustive fields.
#[derive(Deserialize, Debug, Getters)]
pub struct App {
    #[getset(get = "pub")]
    #[serde(rename = "appId")]
    app_id: String,
    #[getset(get = "pub")]
    #[serde(rename = "appName")]
    app_name: String,
    #[getset(get = "pub")]
    #[serde(rename = "middleIcon")]
    middle_icon: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    app_type: i32,
    #[getset(get = "pub")]
    description: Option<String>,
}
impl EhallSession {
    pub fn login_with_user_agent(
        account: &str,
        passwd: &[u8],
        ua: &str,
        login_impl: &EhallLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    ) -> Result<Self, Error> {
        let agent = crate::utils::build_agent_with_user_agent(ua);
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(EhallSession { agent })
    }
    pub fn login(
        account: &str,
        passwd: &[u8],
        login_impl: &EhallLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    ) -> Result<Self, Error> {
        let agent = crate::utils::build_agent();
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(EhallSession { agent })
    }
    pub fn use_app(&self, app_id: &str) -> Result<ureq::Response, Box<ureq::Error>> {
        crate::protocol::ehall::use_app(self, app_id)
    }
    pub fn get_app_list(
        &self,
        search_key: &str,
    ) -> Result<Vec<serde_json::Value>, Box<ureq::Error>> {
        #[derive(Deserialize)]
        struct TmpData {
            #[serde(rename = "hasLogin")]
            has_login: bool,
            data: Vec<serde_json::Value>,
        }
        let r = crate::protocol::ehall::get_app_list(self, search_key)?;
        #[cfg(debug_assertions)]
        let TmpData { has_login, data } =
            crate::utils::print_timed_result(crate::utils::time_it(|| {
                r.into_json().expect("failed to parse json.")
            }));
        #[cfg(not(debug_assertions))]
        let TmpData { has_login, data } = r.into_json().expect("failed to parse json.");
        assert!(has_login);
        Ok(data)
    }
}
impl XL4rsSessionTrait for EhallSession {
    fn has_logged_in(&self) -> bool {
        crate::protocol::ehall::has_logged_in(&self.agent)
    }
}
