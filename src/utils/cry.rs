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

use base64::{DecodeError, Engine};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer};
use percent_encoding::PercentEncode;

pub const X_L4RS_ENC_IV: &[u8; 16] = b"xidianscriptsxdu";

#[inline]
pub fn pkcs7_pad<const BLOCK_SIZE: usize>(data: &[u8]) -> Vec<[u8; BLOCK_SIZE]> {
    let len = data.len();
    let batch = len / BLOCK_SIZE;
    let m = len % BLOCK_SIZE;
    let len2 = BLOCK_SIZE - m;
    let mut r = vec![[0u8; BLOCK_SIZE]; batch + 1];
    let pad_num = ((BLOCK_SIZE - m) % 0xFF) as u8;
    let r_data = r.as_mut_ptr() as *mut u8;
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), r_data, len);
        std::ptr::copy_nonoverlapping(
            vec![pad_num; len2].as_ptr(),
            r_data.add(batch * BLOCK_SIZE + m),
            len2,
        );
    }
    r
}
pub fn aes_enc(padded_data: &[u8], key: &[u8; 16], iv: &[u8]) -> Vec<u8> {
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize128, key, iv, blockmodes::NoPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(padded_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .expect("Encrypt failed");

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .copied(),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    final_result
}

pub fn aes_dec(
    enc_data: &[u8],
    key: &[u8; 16],
    iv: &[u8],
) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize128, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(enc_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .copied(),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

#[inline]
pub fn percent_enc(input: &str) -> PercentEncode {
    percent_encoding::utf8_percent_encode(input, percent_encoding::NON_ALPHANUMERIC)
}
#[inline]
pub fn base64_enc<T: AsRef<[u8]>>(input: T) -> String {
    base64::engine::general_purpose::STANDARD.encode(input)
}
#[inline]
pub fn base64_dec<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    base64::engine::general_purpose::STANDARD.decode(input)
}
#[cfg(feature = "md5")]
#[inline]
pub fn md5_enc<T: AsRef<[u8]>>(input: T) -> [u8; 16] {
    md5::compute(input).0
}
#[inline]
pub fn flatten_bytes<const BLOCK_SIZE: usize>(mut blocks: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
    let (p, l, c) = (blocks.as_mut_ptr(), blocks.len(), blocks.capacity());
    unsafe { Vec::from_raw_parts(p as *mut u8, l * BLOCK_SIZE, c * BLOCK_SIZE) }
}

#[cfg(test)]
mod tests {
    use crate::utils::{aes_dec, aes_enc, base64_dec, base64_enc, flatten_bytes, pkcs7_pad};
    use log::info;

    #[test]
    fn test_pkcs7() {
        let padded = pkcs7_pad::<16>(
            b"xidianscriptsxduxidianscriptsxduxidianscriptsxduxidianscriptsxdu\
            asdasddddddddddddddddddddddddddddd",
        );
        info!(
            "{:?}",
            padded
                .iter()
                .flatten()
                .map(|&u| u as char)
                .collect::<String>()
        );
    }

    #[test]
    fn test_aes_enc() {
        let r = flatten_bytes(pkcs7_pad::<16>(b"password"));
        let enc_r = base64_enc(aes_enc(&r, b"x_l4rsforxdsign.", b"xidianscriptsxdu"));
        println!("{enc_r}",);
        println!(
            "{}",
            aes_dec(
                &base64_dec(enc_r).unwrap(),
                b"x_l4rsforxdsign.",
                b"xidianscriptsxdu"
            )
            .unwrap()
            .iter()
            .map(|c| *c as char)
            .collect::<String>()
        );
    }
}
