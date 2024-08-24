#![allow(unused)]
use core::str;
use std::{thread::sleep, time::Duration};

use jsonwebtoken::{
    decode, decode_header, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // 字段承载了token令牌的payload信息
    // 只能放一些非机密的信息
    sub: String,     // 主题，可选
    company: String, // 公司，可选
    // 必须有的，可以验证的字段。
    // UTC timestamp，一个UNIX时间戳
    // 代表了jwt失效的时间点
    exp: u64,
}

const SECRET_KEY: &[u8] = b"a secret key";

fn main() -> Result<()> {
    // 计算token时效的时间：当前的时间戳加上token有效的时间秒数
    let exp = get_current_timestamp() + 1;
    // 组装token的有效荷载payload
    let my_claims = Claims {
        sub: "我是主题".to_string(),
        company: "我是公司".to_string(),
        exp,
    };
    // 默认算法HS256编码token
    let token1 = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )?;
    println!("默认算法HS256生成的token:{}", token1);

    // 解码token
    let mut validation1 = Validation::new(Algorithm::HS256);
    validation1.sub = Some("我是主题".to_string());
    let token_data1 =
        decode::<Claims>(&token1, &DecodingKey::from_secret(SECRET_KEY), &validation1)?;
    println!("token_data1:{:?}", token_data1);

    // 自定义标头和更改算法编码token
    // 使用算法HS512,设值header.kid字段值
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_string());
    let token2 = encode(&header, &my_claims, &EncodingKey::from_secret(SECRET_KEY))?;
    println!("更改标头和算法生成的token:{}", token2);
    let mut validation2 = Validation::new(Algorithm::HS512);
    validation2.sub = Some("我是主题".to_string());
    let token_data2 =
        decode::<Claims>(&token2, &DecodingKey::from_secret(SECRET_KEY), &validation2)?;
    println!("token_data2:{:?}", token_data2);

    // 使用Rsa算法编码token
    // 实际使用过程中密钥不可变的，需要持久化保存，一般以pem文本文件或者der二进制文件保存
    let mut rng = rand::thread_rng();
    let bit_size = 4096;
    let private_key = RsaPrivateKey::new(&mut rng, bit_size)?;
    let public_key = RsaPublicKey::from(&private_key);
    let private_pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::CRLF)?;
    let public_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::CRLF)?;
    // 使用RSA私钥签名
    let token3 = encode(
        &Header::new(Algorithm::RS256),
        &my_claims,
        &EncodingKey::from_rsa_pem(private_pem.as_bytes())?,
    )?;
    println!("使用rsa算法生成的token:{}", token3);
    // sleep(Duration::from_secs(3));
    let mut validation3 = Validation::new(Algorithm::RS256);
    validation3.sub = Some("我是主题".to_string());
    validation3.leeway = 0;
    let token_data3 = decode::<Claims>(
        &token3,
        &DecodingKey::from_rsa_pem(public_pem.as_bytes())?,
        &validation3,
    )?;
    println!("token_data3:{:?}", token_data3);

    // 解码标头
    // 1.需要分析token使用了什么算法
    // 2.需要了解kid信息
    // 解码标头不灰执行签名验证或者验证任何声明。
    let header3 = decode_header(&token3)?;
    println!("header3:{:?}", header3);

    Ok(())
}
