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

use ureq::Error;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct AgentError(#[from] Box<ureq::Error>);
impl From<ureq::Error> for AgentError {
    fn from(value: ureq::Error) -> Self {
        Self(Box::new(value))
    }
}
impl AgentError {
    pub fn is_fatal(&self) -> bool {
        // Error::Status(_code, _r) => {
        //     //TODO
        //     true
        // }
        // Error::Transport(t) => {
        //     match t.kind() {
        //         // 说明可能是程序 Bug, 故视为致命错误。
        //         ErrorKind::InvalidUrl => true,
        //         // 说明可能是程序 Bug, 故视为致命错误。
        //         ErrorKind::UnknownScheme => true,
        //         //　有时会暂时性地解析失败。所以不视为致命错误。
        //         ErrorKind::Dns => false,
        //         ErrorKind::InsecureRequestHttpsOnly => true,
        //         ErrorKind::ConnectionFailed => true,
        //         ErrorKind::TooManyRedirects => false,
        //         ErrorKind::BadStatus => true,
        //         // 说明可能是程序 Bug, 故视为致命错误。
        //         ErrorKind::BadHeader => true,
        //         ErrorKind::Io => false,
        //         ErrorKind::InvalidProxyUrl => true,
        //         ErrorKind::ProxyConnect => true,
        //         ErrorKind::ProxyUnauthorized => true,
        //         ErrorKind::HTTP => {
        //             //TODO
        //             false
        //         }
        //     }
        // }
        // use ureq_proto::Error as ProtoError;
        match &*self.0 {
            Error::StatusCode(code) => *code != 504,
            Error::Http(_) => true,
            Error::BadUri(_) => true,
            Error::Protocol(_e) => {
                // TODO
                false
            }
            Error::Io(_) => false,
            Error::Timeout(_) => {
                // TODO
                true
            }
            Error::HostNotFound => true,
            Error::RedirectFailed => {
                // TODO
                false
            }
            Error::InvalidProxyUrl => true,
            Error::ConnectionFailed => true,
            Error::BodyExceedsLimit(_) => true,
            Error::TooManyRedirects => false,
            Error::RequireHttpsOnly(_) => false,
            Error::LargeResponseHeader(_, _) => false,
            Error::ConnectProxyFailed(_) => true,
            Error::BodyStalled => true,
            _ => {
                // TODO
                true
            }
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error(transparent)]
    AgentError(#[from] AgentError),
    #[error(transparent)]
    CaptchaError(#[from] CaptchaError),
    #[error("加解密错误：`{0}`.")]
    CryptoError(String),
    #[error("登录失败，服务器返回信息：`{0}`.")]
    ServerError(String),
}
impl LoginError {
    #[inline]
    pub fn is_fatal(&self) -> bool {
        match self {
            LoginError::AgentError(e) => e.is_fatal(),
            LoginError::CaptchaError(e) => match e {
                CaptchaError::AgentError(e) => e.is_fatal(),
                CaptchaError::VerifyFailed => false,
                CaptchaError::Canceled(_) => false,
            },
            LoginError::CryptoError(_) => false,
            LoginError::ServerError(_) => false,
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum CaptchaError {
    #[error(transparent)]
    AgentError(#[from] AgentError),
    #[error("验证失败。")]
    VerifyFailed,
    #[error("操作被主动取消：`{0}`.")]
    Canceled(String),
}
impl CaptchaError {
    #[inline]
    pub fn is_fatal(&self) -> bool {
        match self {
            CaptchaError::AgentError(e) => e.is_fatal(),
            CaptchaError::VerifyFailed => false,
            CaptchaError::Canceled(_) => true,
        }
    }
}
