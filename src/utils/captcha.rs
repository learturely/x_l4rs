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

use crate::protocol::open_slider_captcha;
use crate::utils::{base64_dec, get_now_timestamp_mills};
use cxlib_imageproc::{find_max_ncc, find_sub_image, image_from_bytes};
use log::debug;
use serde::Deserialize;
use ureq::Agent;

pub fn solve_captcha(agent: &Agent) -> u32 {
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
    } = open_slider_captcha(agent, get_now_timestamp_mills())
        .expect("Failed to load captcha slider")
        .into_json()
        .expect("Failed to parse captcha slider");
    let small_image = base64_dec(small_image).expect("Failed to base64 decode captcha small image");
    let big_image = base64_dec(big_image).expect("Failed to base64 decode captcha big image");
    let small_image = image_from_bytes(small_image);
    let big_image = image_from_bytes(big_image);
    let v = find_sub_image(&big_image, &small_image, find_max_ncc);
    debug!("{v}, {}", v * 280 / big_image.width());
    v * 280 / big_image.width()
}

#[cfg(test)]
mod tests {
    use log::info;
    use super::*;
    #[test]
    fn test_solve_captcha() {
        let agent = Agent::new();
        let result = solve_captcha(&agent);
        info!("{}", result);
    }
}
