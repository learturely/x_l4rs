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

use cxlib_error::LoginError;
use log::debug;

/// 找到所需的 html 表单内容。
/// ident 是标识数组，将依次查找标识。
///
/// ``` text
/// <form ...> <div>...</div> </form>
/// ```
pub fn find_form_content<'a>(ident: &[&str], html: &'a str) -> Result<&'a str, LoginError> {
    let mut form_begin = Err(LoginError::ServerError("登录页缺少表单内容".to_string()));
    for ident in ident {
        debug!("{ident}");
        if let Some(s) = html.find(ident) {
            form_begin = Ok(s);
        }
    }
    debug!("{:?}", form_begin);
    let html = &html[form_begin?..];
    let form_end = html.find("</form>").unwrap_or(html.len());
    let html = &html[..form_end];
    let form_begin = html.rfind("</div>").unwrap_or(0);
    let html = &html[form_begin + 6..];
    debug!("{html}");
    Ok(html)
}
/// ``` html
/// <input id="..." name= "..." value="..."/>
/// ```
pub fn find_id_value_pair<'a>(
    ident: &[&str],
    input: &'a str,
) -> Result<(&'a str, &'a str), LoginError> {
    fn find_id_value_pair_internal<'a>(
        ident: &[&str],
        input: &'a str,
    ) -> Option<(&'a str, &'a str)> {
        let mut id_begin = None;
        for ident in ident {
            if let Some(s) = input.find(ident) {
                id_begin = Some(s + ident.len());
            }
        }
        let id = &input[id_begin?..];
        let id_end = id.find("\"")?;
        let id = &id[..id_end];
        let f_begin = input.find("value=\"")?;
        let f = &input[f_begin + 7..];
        let f_end = f.find("\"")?;
        let value = &f[..f_end];
        // TODO: this debug print MAYBE DANGEROUS!!!
        debug!("{id:?}: {value:?}");
        Some((id, value))
    }
    find_id_value_pair_internal(ident, input)
        .ok_or_else(|| LoginError::ServerError("登录页缺少内容".to_string()))
}
#[cfg(feature = "rsbbs")]
pub(crate) mod rsbbs {
    use cxlib_error::{CaptchaError, LoginError};
    use std::ops::Range;

    /// 查找 `id_hash`, 该值与验证码相关。
    ///
    /// updateseccode('id_hash') 会更新二维码，所以该函数会找该函数，并提取其参数。
    pub fn find_id_hash(html: &str) -> Option<&str> {
        let func_s = html.find("updateseccode('")?;
        let html = &html[func_s + 15..];
        let func_e = html.find('\'')?;
        let id_hash = &html[..func_e];
        Some(id_hash)
    }
    /// updateseccode 函数返回的数据中，包含一个 <img> 元素，其 id 为 "vseccode_{id_hash}".
    pub fn find_vcode_img_url<'a>(id_hash: &str, html: &'a str) -> Result<&'a str, LoginError> {
        fn find_vcode_img_url_internal<'a>(id_hash: &str, html: &'a str) -> Option<&'a str> {
            let span_s = html.find(&format!("vseccode_{id_hash}"))?;
            let html = &html[span_s..];
            let img_s = html.rfind("src=\"")?;
            let html = &html[img_s + 5..];
            let img_e = html.find("\"")?;
            let html = &html[..img_e];
            Some(html)
        }
        find_vcode_img_url_internal(id_hash, html).ok_or_else(|| {
            LoginError::CaptchaError(CaptchaError::Canceled(
                "未找到验证码图片，跳过下载。".to_owned(),
            ))
        })
    }
    /// 登录相关，该值用来确定 form id, 为 `loginform_{login_hash}`.
    pub fn find_login_hash(html: &str) -> Result<Range<usize>, LoginError> {
        pub fn find_login_hash_internal(html: &str) -> Option<Range<usize>> {
            let s = html.find("loginhash=")? + 10;
            let login_hash = &html[s..];
            let e = login_hash.find('"');
            if let Some(e) = e {
                Some(s..s + e)
            } else if login_hash.len() >= 5 {
                Some(s..s + 5)
            } else {
                None
            }
        }
        find_login_hash_internal(html)
            .ok_or_else(|| LoginError::ServerError("未找到 `login_hash`.".to_owned()))
    }
    /// 复用了 login_hash 的范围，因为该值出现在 url 内。
    ///
    /// 注意需要将 `&amp;` 转义为 `&`.
    pub fn find_login_url(login_hash_range: Range<usize>, html: &str) -> Result<&str, LoginError> {
        fn find_login_url_internal(login_hash_range: Range<usize>, html: &str) -> Option<&str> {
            let html = &html[..login_hash_range.end];
            let s = html.rfind("action=\"")? + 8;
            Some(&html[s..])
        }
        find_login_url_internal(login_hash_range, html)
            .ok_or_else(|| LoginError::ServerError("未找到登录地址。".to_owned()))
    }
}