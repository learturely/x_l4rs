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

use crate::{
    utils::{
        find_form_content, find_id_value_pair,
        rsbbs::{find_login_hash, find_login_url},
    },
    QuestionAnswerPair,
};
use cxlib_error::{AgentError, LoginError};
use cxlib_imageproc::image_from_bytes;
use image::DynamicImage;
use log::debug;
use std::{fmt::Display, ops::Deref};
use ureq::{Agent, AgentBuilder, Response};

pub enum RSBBSProtocolItem {
    Host,
}
impl RSBBSProtocolItem {
    fn get_default(&self) -> &'static str {
        match self {
            RSBBSProtocolItem::Host => Self::HOST,
        }
    }
}
impl RSBBSProtocolItem {
    pub fn get(&self) -> &'static str {
        self.get_default()
    }
}
impl RSBBSProtocolItem {
    pub const HOST: &'static str = "rs.xidian.edu.cn";
}
impl Display for RSBBSProtocolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get().fmt(f)
    }
}
pub fn login_page(agent: &Agent) -> Result<Response, AgentError> {
    let url = format!(
        "https://{}/member.php?mod=logging&action=login&referer=http%3A%2F%2Frs.xidian.edu.cn%2Fforum.php",
        RSBBSProtocolItem::Host
    );
    Ok(agent.get(&url).call()?)
}
pub fn update_sec_code<const IS_FIRST: bool>(
    agent: &Agent,
    id_hash: &str,
    referer: &str,
) -> Result<Response, AgentError> {
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
    Ok(agent.get(&url).set("Referer", referer).call()?)
}
pub fn refresh_vcode(agent: &Agent, id_hash: &str, referer: &str) -> Result<(), LoginError> {
    update_sec_code::<false>(agent, id_hash, referer)?;
    Ok(())
}
pub fn download_vcode_image(
    agent: &Agent,
    referer: &str,
    img_url: &str,
) -> Result<DynamicImage, AgentError> {
    let url = format!("https://{}/{img_url}", RSBBSProtocolItem::Host);
    let mut v = Vec::new();
    let img = agent.get(&url).set("Referer", referer).call()?;
    // let img_ = unsafe { ptr::read(&img) };
    // let r = img_.into_string().unwrap();
    // debug!("{}", r);
    img.into_reader()
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
) -> Result<Response, LoginError> {
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
        .set("Origin", RSBBSProtocolItem::Host.get().as_ref())
        .set("Referer", referer)
        .send_form(&post_data)
        .map_err(AgentError::from)?)
}
pub fn has_logged_in(agent: &Agent) -> bool {
    // TODO: UserAgent.
    // TODO: use ureq 3.x.
    let agent = AgentBuilder::new()
        .redirects(0)
        .cookie_store(agent.cookie_store().deref().clone())
        .build();
    let url = format!("https://{}/forum.php", RSBBSProtocolItem::Host);
    agent.get(&url).call().is_ok_and(|r| {
        let code = r.status();
        debug!("{code}");
        code != 302
    })
}
