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

use crate::protocol::ehall::EhallProtocolItem;
use ureq::{Agent, Body, http::Response};
#[inline]
pub fn use_app(agent: &Agent, app_id: &str) -> Result<Response<Body>, Box<ureq::Error>> {
    Ok(agent
        .get(&format!("{}?appId={app_id}", EhallProtocolItem::AppShow))
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8",
        )
        .call()?)
}

#[inline]
pub fn get_app_list(agent: &Agent, search_key: &str) -> Result<Response<Body>, Box<ureq::Error>> {
    Ok(agent
        .get(&format!(
            "{}?{}&{}&{}&{}&{}",
            EhallProtocolItem::ServiceSearchCustom,
            format_args!("searchKey={search_key}"),
            "pageNumber=1",
            "pageSize=150",
            "sortKey=recentUseCount",
            "orderKey=desc",
        ))
        .call()?)
}
