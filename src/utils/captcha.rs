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
    protocol::open_slider_captcha,
    utils::{base64_dec, get_now_timestamp_mills},
};
use cxlib_imageproc::image_from_bytes;
use image::DynamicImage;
use log::debug;
use serde::Deserialize;
use ureq::Agent;

pub fn solve_captcha(
    agent: &Agent,
    captcha_solver: &impl Fn(&DynamicImage, &DynamicImage) -> u32,
) -> u32 {
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
    let big_image = image_from_bytes(big_image);
    let small_image = image_from_bytes(small_image);
    let v = captcha_solver(&big_image, &small_image);
    let r = v * 280 / big_image.width();
    debug!("{v}, {r}");
    r
}
