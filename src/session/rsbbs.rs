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

use crate::error::{CaptchaError, LoginError};
use crate::utils::rsbbs::{find_id_hash, find_vcode_img_url};
use crate::{
    LOGIN_RETRY_TIMES, XL4rsSessionTrait,
    protocol::rsbbs::{download_vcode_image, login_page, refresh_vcode, update_sec_code},
    utils::md5_enc,
};
use image::DynamicImage;
use log::{debug, warn};
use std::ops::Deref;
use ureq::{Agent, ResponseExt};

#[derive(Debug, Copy, Clone, Default)]
#[repr(u8)]
#[non_exhaustive]
pub enum Question {
    #[default]
    /// 未设置
    Q0 = 0,
    /// 母亲的名字
    Q1 = 1,
    /// 爷爷的名字,
    Q2 = 2,
    /// 父亲出生的城市,
    Q3 = 3,
    /// 您其中一位老师的名字
    Q4 = 4,
    /// 您个人计算机的型号
    Q5 = 5,
    /// 您最喜欢的餐馆名称
    Q6 = 6,
    /// 驾驶执照最后四位数字
    Q7 = 7,
}
impl Question {
    pub const DEFAULT: Question = Question::Q0;
    #[inline]
    pub fn description(&self) -> &'static str {
        match self {
            Question::Q0 => "未设置",
            Question::Q1 => "母亲的名字",
            Question::Q2 => "爷爷的名字",
            Question::Q3 => "父亲出生的城市",
            Question::Q4 => "您其中一位老师的名字",
            Question::Q5 => "您个人计算机的型号",
            Question::Q6 => "您最喜欢的餐馆名称",
            Question::Q7 => "驾驶执照最后四位数字",
        }
    }
}
impl Question {
    #[inline]
    pub fn get_id(&self) -> u8 {
        unsafe { *(&self as *const _ as *const u8) }
    }
    #[inline]
    pub fn from_id(id: u8) -> Self {
        unsafe { std::mem::transmute(id) }
    }
}
impl From<u8> for Question {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from_id(value)
    }
}
impl From<Question> for u8 {
    #[inline]
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
        question: Question::DEFAULT,
        answer: "",
    };
}
#[derive(Debug, Default)]
pub struct RSBBSLoginImpl<'a> {
    question_answer_pairs: QuestionAnswerPair<'a>,
    cookies_time_days: Option<u32>,
}
impl RSBBSLoginImpl<'_> {
    #[inline]
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
        let referer = login_page.get_uri().to_string();
        let html = login_page
            .into_body()
            .read_to_string()
            .expect("Failed to convert Response into String.");
        let id_hash = find_id_hash(&html)
            .ok_or_else(|| LoginError::ServerError("未找到 `id_hash`, 跳过下载。".to_owned()))?;
        let r = update_sec_code::<true>(agent, id_hash, &referer)?
            .into_body()
            .read_to_string()
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
            .into_body()
            .read_to_string()
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
impl RSBBSSession {
    #[inline]
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
    #[inline]
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
    #[inline]
    fn has_logged_in(&self) -> bool {
        crate::protocol::rsbbs::has_logged_in(&self.agent)
    }
}
