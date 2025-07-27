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

use crate::{
    IDSLoginImpl, XL4rsSessionTrait,
    error::{CaptchaError, LoginError},
};
use getset2::Getset2;
use image::DynamicImage;
use serde::Deserialize;
use std::ops::Deref;
use ureq::{Agent, Body, http::Response};

pub struct EhallLoginImpl {
    inner: IDSLoginImpl,
}

impl EhallLoginImpl {
    #[inline]
    pub fn new() -> Self {
        EhallLoginImpl {
            inner: IDSLoginImpl::TARGET_EHALL,
        }
    }
}
impl From<IDSLoginImpl> for EhallLoginImpl {
    #[inline]
    fn from(value: IDSLoginImpl) -> Self {
        Self { inner: value }
    }
}
impl Default for EhallLoginImpl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for EhallLoginImpl {
    type Target = IDSLoginImpl;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EhallSession {
    agent: Agent,
}
impl Deref for EhallSession {
    type Target = Agent;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
// TODO: non_exhaustive fields.
#[derive(Deserialize, Debug, Getset2)]
#[getset2(get_ref(pub))]
pub struct App {
    #[serde(rename = "appId")]
    app_id: String,
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "middleIcon")]
    middle_icon: String,
    #[serde(rename = "type")]
    app_type: i32,
    description: Option<String>,
}
impl EhallSession {
    #[inline]
    pub fn login_with_user_agent(
        account: &str,
        passwd: &[u8],
        ua: &str,
        login_impl: &EhallLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, CaptchaError>,
    ) -> Result<Self, LoginError> {
        let agent = crate::utils::build_agent_with_user_agent(ua);
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(EhallSession { agent })
    }
    #[inline]
    pub fn login(
        account: &str,
        passwd: &[u8],
        login_impl: &EhallLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, CaptchaError>,
    ) -> Result<Self, LoginError> {
        let agent = crate::utils::build_agent();
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(EhallSession { agent })
    }
    #[inline]
    pub fn use_app(&self, app_id: &str) -> Result<Response<Body>, Box<ureq::Error>> {
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
        let TmpData { has_login, data } = crate::utils::time_it_and_print_result(|| {
            r.into_body().read_json().expect("failed to parse json.")
        });
        assert!(has_login);
        Ok(data)
    }
}
impl XL4rsSessionTrait for EhallSession {
    #[inline]
    fn has_logged_in(&self) -> bool {
        crate::protocol::ehall::has_logged_in(&self.agent)
    }
}
