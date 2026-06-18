//! 微信相关的加密解密工具
//!
//! 提供 AES-128-CBC 解密、HMAC-SHA256 签名等功能

use crate::{Error, Result};
use aes::{
    Aes128,
    cipher::{Array, BlockModeDecrypt, KeyIvInit, block_padding::Pkcs7},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use cbc::Decryptor;
use hex::encode;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type Aes128CbcDec = Decryptor<Aes128>;

/// 使用 AES-128-CBC 算法解密数据，数据采用 PKCS#7 填充
///
/// # 参数
///
/// - `encrypted_data`: base64 编码的加密数据
/// - `session_key`: base64 编码的会话密钥
/// - `iv`: base64 编码的初始向量
///
/// # 返回
///
/// 解密后的字节数据
pub fn aes_decrypt(encrypted_data: &str, session_key: &str, iv: &str) -> Result<Vec<u8>> {
    let key = STANDARD.decode(session_key)?;
    let iv = STANDARD.decode(iv)?;
    let key =
        Array::slice_as_array(&key).ok_or(Error::InvalidParameter("invalid key".to_string()))?;
    let iv = Array::slice_as_array(&iv).ok_or(Error::InvalidParameter("invalid iv".to_string()))?;
    let encrypted_data = STANDARD.decode(encrypted_data)?;

    let decryptor = Aes128CbcDec::new(&key, &iv);

    Ok(decryptor.decrypt_padded_vec::<Pkcs7>(&encrypted_data)?)
}

/// 使用 HMAC-SHA256 算法签名数据
///
/// # 参数
///
/// - `data`: 要签名的数据
/// - `key`: 签名密钥
///
/// # 返回
///
/// hex 编码的签名
pub fn hmac_sha256(data: &[u8], key: &str) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())?;
    mac.update(data);
    let hasher = mac.finalize();
    Ok(encode(hasher.into_bytes()))
}
