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

use crate::protocol::XL4rsProtocolItem;
use cxlib_protocol::ProtocolItemTrait;
use ureq::Agent;

pub fn check_need_captcha(
    agent: &Agent,
    uname: &str,
    time_stamp_mills: u128,
) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!(
            "{}?username={}&_={}",
            XL4rsProtocolItem::CheckNeedCaptcha,
            uname,
            time_stamp_mills
        ))
        .call()?)
}

pub fn open_slider_captcha(
    agent: &Agent,
    time_stamp_mills: u128,
) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!(
            "{}?_={time_stamp_mills}",
            XL4rsProtocolItem::OpenSliderCaptcha
        ))
        .call()?)
}
pub fn verify_slider_captcha(
    agent: &Agent,
    move_length: u32,
) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .post(&XL4rsProtocolItem::VerifySliderCaptcha.get())
        .set("Refer", "https://ids.xidian.edu.cn/authserver/login")
        .send_form(&[
            ("canvasLength", "280"),
            ("moveLength", move_length.to_string().as_str()),
        ])?)
}
