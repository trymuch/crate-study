use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,     // 主题
    company: String, // 公司
    // 必须有的，可以验证的字段。
    // UTC timestamp，一个UNIX时间戳
    // 代表了jwt失效的时间点
    exp: i64,
}

const SECRET_KEY: &[u8] = b"a secret key";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let now = Utc::now();
    println!("now:{}", now);
    let exp = match now.checked_add_signed(TimeDelta::seconds(1)) {
        Some(dt) => dt,
        None => Err("不能生成时间戳")?,
    }
    .timestamp_millis();
    println!("exp:{}", exp);
    let claims = Claims {
        sub: "我是主题".to_owned(),
        company: "翌圣".to_owned(),
        exp,
    };
    println!("claims:{:?}",claims);
    // 使用默认的HS256算法生成的token
    let token1 = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )?;
    println!("token1:{}", token1);

    // 自定义头部(算法)生成token
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let token2 = encode(&header, &claims, &EncodingKey::from_secret(SECRET_KEY))?;
    println!("token2:{}", token2);

    sleep(Duration::from_secs(5));

    let mut validation = Validation::new(Algorithm::HS512);
    validation.sub = Some("我是主题".to_owned());
    validation.set_required_spec_claims(&["exp"]);
    validation.validate_exp = false;
    let token_data =
        decode::<Claims>(&token2, &DecodingKey::from_secret(SECRET_KEY), &validation)?;

    println!("token_data:{:?}", token_data);
    println!("{:?}", start.elapsed());
    Ok(())
}
