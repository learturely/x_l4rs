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

const X_L4RS_ENC_KEY: &[u8; 16] = b"x_l4rsforxdsign.";
use crate::utils::{aes_dec, aes_enc, base64_dec, base64_enc, X_L4RS_ENC_IV};
use crate::IDSLoginImpl;
use cxlib_error::Error;
#[cfg(feature = "cxlib_protocol")]
use cxlib_protocol::{ProtocolItem, ProtocolItemTrait};
use cxlib_utils::pkcs7_pad;
use image::DynamicImage;
use ureq::Agent;

pub struct XL4rsLoginSolver<CaptchaSolver>
where
    CaptchaSolver: Fn(&DynamicImage, &DynamicImage) -> u32,
{
    inner: IDSLoginImpl,
    captcha_solver: CaptchaSolver,
}
impl<CaptchaSolver> XL4rsLoginSolver<CaptchaSolver>
where
    CaptchaSolver: Fn(&DynamicImage, &DynamicImage) -> u32,
{
    pub fn new(inner: IDSLoginImpl, captcha_solver: CaptchaSolver) -> Self {
        XL4rsLoginSolver {
            inner,
            captcha_solver,
        }
    }
}
impl<CaptchaSolver> cxlib_login::LoginSolverTrait for XL4rsLoginSolver<CaptchaSolver>
where
    CaptchaSolver: Fn(&DynamicImage, &DynamicImage) -> u32 + Send + Sync + 'static,
{
    fn login_type(&self) -> &str {
        "xd"
    }

    fn is_logged_in(&self, agent: &Agent) -> bool {
        crate::protocol::ids::has_logged_in(agent)
    }

    fn login_s(&self, account: &str, enc_passwd: &str) -> Result<Agent, Error> {
        let passwd = aes_dec(
            &base64_dec(enc_passwd)
                .map_err(|e| Error::LoginError(format!("密码解密出错：{e:?}")))?,
            X_L4RS_ENC_KEY,
            X_L4RS_ENC_IV,
        )
        .map_err(|e| Error::LoginError(format!("密码解密出错：{e:?}")))?;
        #[cfg(not(feature = "cxlib_protocol"))]
        let agent = crate::utils::build_agent();
        #[cfg(feature = "cxlib_protocol")]
        let agent = crate::utils::build_agent_with_user_agent(&ProtocolItem::UserAgent.get());
        self.inner
            .login(&agent, account, &passwd, &self.captcha_solver)?;
        Ok(agent)
    }

    fn pwd_enc(&self, pwd: String) -> Result<String, Error> {
        let padded_pwd = pkcs7_pad::<16>(pwd.as_bytes());
        Ok(base64_enc(aes_enc(
            &padded_pwd.into_iter().flatten().collect::<Vec<_>>(),
            X_L4RS_ENC_KEY,
            X_L4RS_ENC_IV,
        )))
    }
}
