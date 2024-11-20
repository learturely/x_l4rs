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

use base64::{DecodeError, Engine};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer};
use percent_encoding::PercentEncode;

pub(crate) const ENC_IV: &[u8; 16] = b"xidianscriptsxdu";
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

pub fn percent_enc(input: &str) -> PercentEncode {
    percent_encoding::utf8_percent_encode(input, percent_encoding::NON_ALPHANUMERIC)
}
pub fn base64_enc<T: AsRef<[u8]>>(input: T) -> String {
    base64::engine::general_purpose::STANDARD.encode(input)
}
pub fn base64_dec<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    base64::engine::general_purpose::STANDARD.decode(input)
}

pub fn flatten_bytes<const BLOCK_SIZE: usize>(blocks: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
    let (p, l, c) = blocks.into_raw_parts();
    unsafe { Vec::from_raw_parts(p as *mut u8, l * BLOCK_SIZE, c * BLOCK_SIZE) }
}

#[cfg(test)]
mod tests {
    use crate::utils::{aes_dec, aes_enc, base64_dec, base64_enc, flatten_bytes};
    use cxlib_utils::pkcs7_pad;
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
        println!("{enc_r}", );
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
