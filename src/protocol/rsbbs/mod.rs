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
    QuestionAnswerPair,
    error::{AgentError, LoginError},
    utils::{
        find_form_content, find_id_value_pair, image_from_bytes,
        rsbbs::{find_login_hash, find_login_url},
    },
};
use image::DynamicImage;
use log::debug;
use std::{fmt::Display, io::Read};
use ureq::{Agent, Body, http::Response};

pub enum RSBBSProtocolItem {
    Host,
}
impl RSBBSProtocolItem {
    #[inline]
    fn get_default(&self) -> &'static str {
        match self {
            RSBBSProtocolItem::Host => Self::HOST,
        }
    }
}
impl RSBBSProtocolItem {
    #[inline]
    pub fn get(&self) -> &'static str {
        self.get_default()
    }
}
impl RSBBSProtocolItem {
    pub const HOST: &'static str = "rs.xidian.edu.cn";
}
impl Display for RSBBSProtocolItem {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
#[inline]
pub fn login_page(agent: &Agent) -> Result<Response<Body>, AgentError> {
    let url = format!(
        "https://{}/member.php?mod=logging&action=login&referer=http%3A%2F%2Frs.xidian.edu.cn%2Fforum.php",
        RSBBSProtocolItem::Host
    );
    Ok(agent.get(&url).call()?)
}
#[inline]
pub fn update_sec_code<const IS_FIRST: bool>(
    agent: &Agent,
    id_hash: &str,
    referer: &str,
) -> Result<Response<Body>, AgentError> {
    let modid = if IS_FIRST {
        "member%3A%3Alogging"
    } else {
        "undefined"
    };
    let url = format!(
        "https://{0}/misc.php?mod=seccode&action=update&idhash={id_hash}&{1}&modid={modid}",
        RSBBSProtocolItem::Host,
        rand::random_range(0.0f64..=1.0),
    );
    debug!("{url}");
    Ok(agent.get(&url).header("Referer", referer).call()?)
}
#[inline]
pub fn refresh_vcode(agent: &Agent, id_hash: &str, referer: &str) -> Result<(), LoginError> {
    update_sec_code::<false>(agent, id_hash, referer)?;
    Ok(())
}
#[inline]
pub fn download_vcode_image(
    agent: &Agent,
    referer: &str,
    img_url: &str,
) -> Result<DynamicImage, AgentError> {
    let url = format!("https://{}/{img_url}", RSBBSProtocolItem::Host);
    let mut v = Vec::new();
    let img = agent.get(&url).header("Referer", referer).call()?;
    // let img_ = unsafe { ptr::read(&img) };
    // let r = img_.into_string().unwrap();
    // debug!("{}", r);
    img.into_body()
        .into_reader()
        .read_to_end(&mut v)
        .expect("failed to read vcode image.");
    let img = image_from_bytes(v);
    Ok(img)
}
pub fn login(
    agent: &Agent,
    referer: &str,
    (uname, pwd_md5): (&str, &str),
    QuestionAnswerPair { question, answer }: QuestionAnswerPair,
    vcode: &str,
    cookies_time_days: Option<u32>,
    html: &str,
) -> Result<Response<Body>, LoginError> {
    let login_hash = find_login_hash(html)?;
    let login_url = find_login_url(login_hash.clone(), html)?;
    let login_url = login_url.replace("&amp;", "&");
    let url = format!("https://{}/{}", RSBBSProtocolItem::Host, login_url);
    let login_hash = &html[login_hash];
    debug!("login_hash = {}", login_hash);
    let form_id = format!("loginform_{login_hash}");
    let inputs = find_form_content(&[&form_id, "loginform_"], html)?.split("<input ");
    let mut post_data = inputs
        .into_iter()
        .filter_map(|s| {
            let (name, value) = find_id_value_pair(&["name=\""], s).ok()?;
            if ["formhash", "referer", "seccodehash"].contains(&name) {
                Some((name.trim(), value.trim()))
            } else {
                None
            }
        })
        .collect::<Vec<(_, _)>>();
    let question_id = question.get_id().to_string();
    let cookies_time = (cookies_time_days.unwrap_or(30) as u64 * 24 * 60 * 60).to_string();
    post_data.push(("username", uname));
    post_data.push(("password", pwd_md5));
    post_data.push(("questionid", &question_id));
    post_data.push(("answer", answer));
    post_data.push(("seccodemodid", "member%3A%3Alogging"));
    post_data.push(("seccodeverify", vcode));
    post_data.push(("cookietime", &cookies_time));
    Ok(agent
        .post(&url)
        .header("Origin", RSBBSProtocolItem::Host.get())
        .header("Referer", referer)
        .send_form(post_data)
        .map_err(AgentError::from)?)
}
#[inline]
pub fn has_logged_in(agent: &Agent) -> bool {
    let url = format!("https://{}/forum.php", RSBBSProtocolItem::Host);
    agent
        .get(&url)
        .config()
        .max_redirects(0)
        .build()
        .call()
        .is_ok_and(|r| {
            let code = r.status();
            debug!("{code}");
            code != 302
        })
}
