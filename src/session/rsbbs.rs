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

use crate::utils::rsbbs::{find_id_hash, find_vcode_img_url};
use crate::{
    protocol::rsbbs::{download_vcode_image, login_page, refresh_vcode, update_sec_code},
    utils::md5_enc,
    XL4rsSessionTrait, LOGIN_RETRY_TIMES,
};
use cxlib_error::{CaptchaError, LoginError, MaybeFatalError};
use image::DynamicImage;
use log::{debug, warn};
use std::ops::Deref;
use ureq::Agent;

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
pub enum Question {
    #[default]
    未设置 = 0,
    母亲的名字 = 1,
    爷爷的名字 = 2,
    父亲出生的城市 = 3,
    您其中一位老师的名字 = 4,
    您个人计算机的型号 = 5,
    您最喜欢的餐馆名称 = 6,
    驾驶执照最后四位数字 = 7,
}
impl Question {
    pub fn get_id(&self) -> u8 {
        unsafe { *(&self as *const _ as *const u8) }
    }
    pub fn from_id(id: u8) -> Self {
        unsafe { std::mem::transmute(id) }
    }
}
impl From<u8> for Question {
    fn from(value: u8) -> Self {
        Self::from_id(value)
    }
}
impl From<Question> for u8 {
    fn from(value: Question) -> Self {
        value.get_id()
    }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct QuestionAnswerPair<'a> {
    pub question: Question,
    pub answer: &'a str,
}
impl QuestionAnswerPair<'_> {
    pub const DEFAULT: Self = Self {
        question: Question::未设置,
        answer: "",
    };
}
#[derive(Debug, Default)]
pub struct RSBBSLoginImpl<'a> {
    question_answer_pairs: QuestionAnswerPair<'a>,
    cookies_time_days: Option<u32>,
}
impl RSBBSLoginImpl<'_> {
    pub fn new(
        question_answer_pairs: QuestionAnswerPair,
        cookies_time_days: Option<u32>,
    ) -> RSBBSLoginImpl {
        RSBBSLoginImpl {
            question_answer_pairs,
            cookies_time_days,
        }
    }
}

impl RSBBSLoginImpl<'_> {
    pub fn login(
        &self,
        agent: &Agent,
        uname: &str,
        passwd: &[u8],
        vcode_solver: &impl Fn(&DynamicImage) -> Result<String, CaptchaError>,
    ) -> Result<(), LoginError> {
        let login_page = login_page(agent)?;
        let referer = login_page.get_url().to_string();
        let html = login_page
            .into_string()
            .expect("Failed to convert Response into String.");
        let id_hash = find_id_hash(&html)
            .ok_or_else(|| LoginError::ServerError("未找到 `id_hash`, 跳过下载。".to_owned()))?;
        let r = update_sec_code::<true>(agent, id_hash, &referer)?
            .into_string()
            .expect("Failed to convert Response into String.");
        debug!("{r}");
        let img_url = find_vcode_img_url(id_hash, &r)?;
        let pwd = hex::encode(md5_enc(passwd));
        for i in 0..=LOGIN_RETRY_TIMES {
            let img = download_vcode_image(agent, &referer, img_url)?;
            let vcode = vcode_solver(&img);
            let vcode = match vcode {
                Ok(vcode) => vcode,
                Err(e) => {
                    if e.is_fatal() {
                        return Err(e)?;
                    } else {
                        continue;
                    }
                }
            };
            let login_result = crate::protocol::rsbbs::login(
                agent,
                &referer,
                (uname, &pwd),
                self.question_answer_pairs,
                &vcode,
                self.cookies_time_days,
                &html,
            )?
            .into_string()
            .expect("Failed to convert response into string.");
            debug!("{login_result}");
            // 登录成功。
            if login_result.contains("欢迎您回来") {
                break;
            }
            // 验证码错误，默认重试。
            else if login_result.contains("抱歉，验证码填写错误") {
                if i == LOGIN_RETRY_TIMES {
                    return Err(LoginError::CaptchaError(CaptchaError::VerifyFailed));
                } else {
                    warn!("验证码填写错误，请重试。");
                    refresh_vcode(agent, id_hash, &referer)?;
                    continue;
                }
            }
            // 其他错误，如密码错误等，直接返回。
            else {
                let err_msg = login_result
                    .find("![CDATA[")
                    .and_then(|s| login_result.find("<script").map(|e| &login_result[s..e]));
                let err_msg = if let Some(err_msg) = err_msg {
                    err_msg.strip_prefix("抱歉，").unwrap_or(err_msg)
                } else {
                    &login_result
                };
                return Err(LoginError::ServerError(format!("登录失败：{err_msg}",)));
            }
        }
        Ok(())
    }
}
pub struct RSBBSSession {
    agent: Agent,
}

impl Deref for RSBBSSession {
    type Target = Agent;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
impl RSBBSSession {
    pub fn login_with_user_agent(
        account: &str,
        passwd: &[u8],
        ua: &str,
        login_impl: &RSBBSLoginImpl,
        vcode_solver: &impl Fn(&DynamicImage) -> Result<String, CaptchaError>,
    ) -> Result<Self, LoginError> {
        let agent = crate::utils::build_agent_with_user_agent(ua);
        login_impl.login(&agent, account, passwd, vcode_solver)?;
        Ok(RSBBSSession { agent })
    }
    pub fn login(
        account: &str,
        passwd: &[u8],
        login_impl: &RSBBSLoginImpl,
        vcode_solver: &impl Fn(&DynamicImage) -> Result<String, CaptchaError>,
    ) -> Result<Self, LoginError> {
        let agent = crate::utils::build_agent();
        login_impl.login(&agent, account, passwd, vcode_solver)?;
        Ok(RSBBSSession { agent })
    }
}
impl XL4rsSessionTrait for RSBBSSession {
    fn has_logged_in(&self) -> bool {
        crate::protocol::rsbbs::has_logged_in(&self.agent)
    }
}
