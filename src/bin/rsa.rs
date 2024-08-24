#![allow(unused)]
use core::str;
use jsonwebtoken::TokenData;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::pss::BlindedSigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // pkcs1v15_encrypt_study()?;
    // oaep_encrypt_study()?;
    // pkcs1v15_signature_study()?;
    // pss_signature_study()?;
    pkcs1_key_encode_study()?;
    Ok(())
}
// pkcs1v15加密
fn pkcs1v15_encrypt_study() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 4096)?;
    println!("私钥:{:#?}", private_key);
    let public_key = RsaPublicKey::from(&private_key);
    println!("公钥:{:#?}", public_key);

    // 准备需要加密的文本数据

    // unicode编码的字符串切片
    let text = "我是需要加密的一段文本。";
    println!("需要加密的文本:{}", text);

    // 字节数组形式的数据，加密用的数据
    // 无法直接对字符串进行加密，需要转换成字节数组
    let data = text.as_bytes();
    println!("要加密的数据:{:?}", data);

    // 额外知识点：字节数组可以转换成字符串
    // 使用rust核心库str，保证内存安全
    let text_from_data = str::from_utf8(data)?;
    println!("字节数组转换来的字符串文本:{}", text_from_data);

    // 加密encrypt
    // 加密使用公钥，然后使用对应的私钥才能解密
    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)?;
    println!("加密后的数据:{:?}", enc_data);

    // 解密decrypt
    // 使用加密用的公钥对应的私钥解密
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data)?;
    println!("解密后的数据:{:?}", dec_data);
    assert_eq!(data, &dec_data);
    Ok(())
}

// oaep加密
fn oaep_encrypt_study() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let bits = 4096;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    // 加密
    let data = b"hello, world!";
    println!("要加密的数据:{:?}", data);
    let padding = Oaep::new::<Sha256>();
    let enc_data = public_key.encrypt(&mut rng, padding, data)?;
    println!("加密后的数据:{:?}", enc_data);
    // 解密
    let padding = Oaep::new::<Sha256>();
    let dec_data = private_key.decrypt(padding, &enc_data)?;
    println!("解密后的数据:{:?}", dec_data);

    Ok(())
}

// pkcs1v15签名
fn pkcs1v15_signature_study() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let bits = 4096;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    let signing_key = SigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();

    // 签名
    let data = b"hello,world!";
    println!("要签名的数据:{:?}", data);
    let signature = signing_key.sign_with_rng(&mut rng, data);
    println!("签名后的数据:{:?}", signature.to_bytes().as_ref());

    // 验签
    verifying_key.verify(data, &signature)?;

    Ok(())
}
// pss签名
fn pss_signature_study() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let bits = 4096;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let signing_key = BlindedSigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();
    // 签名
    let data = b"hello,world!";
    println!("要签名的数据:{:?}", data);
    let signature = signing_key.sign_with_rng(&mut rng, data);
    println!("签名后的数据:{:?}", signature.to_bytes().as_ref());
    // 验证签名
    verifying_key.verify(data, &signature)?;
    Ok(())
}

// pkcs#1/pkcs#8 rsa密钥编码/解码
fn pkcs1_key_encode_study() -> Result<(), Box<dyn std::error::Error>> {
    let public_pem = include_str!("../../pkcs#8_public_key.pem");
    println!("public pem:{}", public_pem);

    let public_key = RsaPublicKey::from_public_key_pem(public_pem)?;
    println!("public key:{:#?}", public_key);

    let encode_public_key = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::CRLF)?;

    dbg!(encode_public_key);

    let private_pem = include_str!("../../pkcs#8_private_key.pem");
    println!("private pem:{}", private_pem);
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_pem)?;
    println!("private key:{:#?}", private_key);

    let encode_private_key = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::CRLF)?;
    dbg!(encode_private_key.as_str());

    Ok(())
}

#[cfg(test)]
mod tests {}
