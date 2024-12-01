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

use crate::utils::{find_form_content, find_id_value_pair};
use crate::{
    protocol::ids as ids_protocol,
    utils::{aes_enc, base64_dec, base64_enc, get_now_timestamp_mills, X_L4RS_ENC_IV},
    XL4rsSessionTrait, LOGIN_RETRY_TIMES,
};
use cxlib_error::Error;
use cxlib_imageproc::image_from_bytes;
use cxlib_utils::pkcs7_pad;
use image::DynamicImage;
use log::{debug, warn};
use rand::Rng;
use serde::Deserialize;
use std::ops::Deref;
use ureq::Agent;

fn solve_captcha(
    agent: &Agent,
    captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
) -> Result<u32, Error> {
    #[derive(Deserialize)]
    struct Images {
        #[serde(rename = "smallImage")]
        small_image: String,
        #[serde(rename = "bigImage")]
        big_image: String,
    }
    let Images {
        small_image,
        big_image,
    } = ids_protocol::open_slider_captcha(agent, get_now_timestamp_mills())?
        .into_json()
        .expect("Failed to parse captcha slider");
    let small_image = base64_dec(small_image).expect("Failed to base64 decode captcha small image");
    let big_image = base64_dec(big_image).expect("Failed to base64 decode captcha big image");
    let big_image = image_from_bytes(big_image);
    let small_image = image_from_bytes(small_image);
    let v = captcha_solver(&big_image, &small_image)?;
    let r = v * 280 / big_image.width();
    debug!("{v}, {r}");
    Ok(r)
}
#[derive(Eq, PartialEq)]
pub struct IDSLoginImpl {
    target: &'static str,
}
impl IDSLoginImpl {
    pub fn new(target: &'static str) -> IDSLoginImpl {
        IDSLoginImpl { target }
    }
    pub fn target(&self) -> &'static str {
        self.target
    }
    pub const TARGET_LEARNING: Self = Self {
        target: "https://learning.xidian.edu.cn/cassso/xidian",
    };
    pub const TARGET_EHALL: Self = Self {
        target:
            "http://ehall.xidian.edu.cn/login?service=http://ehall.xidian.edu.cn/new/index.html",
    };
    #[cfg(feature = "cxlib_login")]
    pub fn get_login_solver<CaptchaSolver>(
        self,
        captcha_solver: CaptchaSolver,
    ) -> crate::XL4rsLoginSolver<CaptchaSolver>
    where
        CaptchaSolver: Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    {
        crate::XL4rsLoginSolver::new(self, captcha_solver)
    }
    pub fn login(
        &self,
        agent: &Agent,
        account: &str,
        passwd: &[u8],
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    ) -> Result<(), Error> {
        let page = ids_protocol::login_page(agent, self.target)?
            .into_string()
            .expect("登录页获取失败。");
        for i in 0..=LOGIN_RETRY_TIMES {
            let r = ids_protocol::check_need_captcha(agent, account, get_now_timestamp_mills());
            let r = r
                .map_err(Error::from)
                .and_then(|r| {
                    let r = r.into_string().expect("反序列化错误。");
                    debug!("{r}");
                    if r.contains('t') {
                        #[derive(Deserialize)]
                        struct Tmp {
                            #[serde(rename = "errorMsg")]
                            error_msg: String,
                        }
                        let v = solve_captcha(agent, captcha_solver)?;
                        let Tmp { error_msg } = ids_protocol::verify_slider_captcha(agent, v)?
                            .into_json()
                            .expect("json parse failed.");
                        debug!("{error_msg}");
                        if error_msg == "success" {
                            Ok(())
                        } else {
                            Err(Error::LoginError(format!(
                                "滑块验证失败：{error_msg}，请重新登录。"
                            )))
                        }
                    } else {
                        Ok(())
                    }
                })
                .map_err(|e| Error::LoginError(format!("滑块验证失败：{e}，请重新登录。")));
            match r {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    if i == LOGIN_RETRY_TIMES {
                        return Err(e);
                    } else {
                        warn!("{e}");
                        continue;
                    }
                }
            }
        }
        let inputs =
            find_form_content(&["id=\"pwdLoginDiv\"", "id=\"pwdFromId\""], &page)?.split("<input ");
        let mut key = None;
        let mut post_data = inputs
            .into_iter()
            .filter_map(|s| {
                let (id, value) = find_id_value_pair(&["id=\"", "name=\""], s).ok()?;
                if id == "pwdEncryptSalt" {
                    let mut k = [0; 16];
                    k.copy_from_slice(value.as_bytes());
                    key = Some(k);
                    None
                } else {
                    Some((id.trim(), value.trim()))
                }
            })
            .collect::<Vec<(_, _)>>();
        let password = {
            let mut n = vec![*X_L4RS_ENC_IV; 4]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let mut padded_pwd = pkcs7_pad::<16>(passwd)
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            n.append(&mut padded_pwd);
            // 因为 CBC 加密的特点，解密出最后一个数据块和倒数第一个后，即可获取倒数第一个数据块的明文，也就是说 ENC_IV 可以完全随机。
            // 但也许对方会验证一下 ENC_IV 是否符合一定标准。
            // 或许可以使用以下函数，不过暂时使用固定值。
            fn __gen_bytes<const SIZE: usize>() -> [u8; SIZE] {
                let mut bytes = [0; SIZE];
                for b in bytes.iter_mut() {
                    let a = rand::thread_rng().gen_range(0..AES_CHARS_LEN);
                    *b = AES_CHARS[a];
                }
                static AES_CHARS: &[u8; 48] = b"ABCDEFGHJKMNPQRSTWXYZabcdefhijkmnprstwxyz2345678";
                static AES_CHARS_LEN: usize = 48;
                bytes
            }
            aes_enc(
                &n,
                &key.ok_or_else(|| Error::LoginError("密码无法加密！".to_string()))?,
                &n[..16],
            )
        };
        let password = base64_enc(password);
        post_data.push(("username", account));
        post_data.push(("password", &password));
        post_data.push(("remember_me", "true"));
        post_data.push(("captcha", ""));
        let _ = ids_protocol::login(agent, self.target, &post_data)?;
        Ok(())
    }
}

pub struct IDSSession {
    agent: Agent,
}

impl Deref for IDSSession {
    type Target = Agent;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
impl IDSSession {
    pub fn login_with_user_agent(
        account: &str,
        passwd: &[u8],
        ua: &str,
        login_impl: &IDSLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    ) -> Result<Self, Error> {
        let agent = crate::utils::build_agent_with_user_agent(ua);
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(IDSSession { agent })
    }
    pub fn login(
        account: &str,
        passwd: &[u8],
        login_impl: &IDSLoginImpl,
        captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> Result<u32, Error>,
    ) -> Result<Self, Error> {
        let agent = crate::utils::build_agent();
        login_impl.login(&agent, account, passwd, captcha_solver)?;
        Ok(IDSSession { agent })
    }
}
impl XL4rsSessionTrait for IDSSession {
    fn has_logged_in(&self) -> bool {
        crate::protocol::ids::has_logged_in(&self.agent)
    }
}
