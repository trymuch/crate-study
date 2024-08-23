# 各种crate的学习

## chrono -- 操作日期时间的crate

### TimeDelta/ Duration -- 时间段

TimeDelta结构体，别名是Duration，是对”时间段“的抽象，表示一个一段精确的时间跨度，以秒和纳秒表示。
TimeDelta和标准库中的Duration的区别在于其是一个有符号的值（可以是负值），标准库中的Duration是一个无符号的值（不能是负值）。
TimeDelta实现了Clone,Copy, Debug, Display, Default, Eq, PartialEq, Ord, PartialOrd等trait,因此可以复制、克隆、打印、生成默认值以及比较。
TimeDelta实现了Add, Sub等运算trait，因此可以对其实例进行运算。

```rust
    // 1.创建一个时间段
    // 1.1 new -> Option(TimeDelta)
    let duration =
        TimeDelta::new(10, 100).expect("duration is out of bounds, or nanos >= 1,000,000,000 ");
    println!("duration: {:?}", duration);
    println!("duration: {}", duration);

    // 1.2 weeks,days,hours,minutes,seconds,milliseconds,microseconds,nanoseconds -> TimeDelta
    let weeks = TimeDelta::weeks(1);
    println!("weeks: {}", weeks);
    let days = TimeDelta::days(7);
    println!("days: {}", days);
    let hours = TimeDelta::hours(7 * 24);
    println!("hours: {}", hours);
    let minutes = TimeDelta::minutes(7 * 24 * 60);
    println!("minutes: {}", minutes);
    let seconds = TimeDelta::seconds(7 * 24 * 60 * 60);
    println!("seconds: {}", seconds);
    let milliseconds = TimeDelta::milliseconds(7 * 24 * 60 * 60 * 1000);
    println!("milliseconds: {}", milliseconds);
    let microseconds = TimeDelta::microseconds(7 * 24 * 60 * 60 * 1000 * 1000);
    println!("microseconds: {}", microseconds);
    let nanoseconds = TimeDelta::nanoseconds(7 * 24 * 60 * 60 * 1000 * 1000 * 1000);
    println!("nanoseconds: {}", nanoseconds);

    // 1.3 try_*系列 -> Option<TimeDelta>
    let try_days = TimeDelta::try_days(7).expect("duration is out of bounds");
    println!("try_days:  {}", try_days);
    println!("try_days: {:?}", try_days);

    // 1.4 zero 创建一个零时间段/ is_zero 判断一个时间段是否为零时间段
    let zero = TimeDelta::zero();
    println!("zero: {}", zero);
    println!("is_zero: {}", zero.is_zero());

    // 2. 从时间段中获取各种时间单位的总数 num_*系列方法 -> i64
    let one_week_and_three_days = TimeDelta::weeks(1) + TimeDelta::days(3);
    println!("one_week_and_three_days: {}", one_week_and_three_days);
    let num_weeks = one_week_and_three_days.num_weeks();
    println!("num_weeks: {}", num_weeks);
    let num_days = one_week_and_three_days.num_days();
    println!("num_days: {}", num_days);

    // 3. 和标准库Duration相互转换 to_std/ from_std
    let two_hours_ten_minutes = TimeDelta::hours(2) + TimeDelta::minutes(10);
    let std_duration = two_hours_ten_minutes.to_std().unwrap();
    println!("two_hours_ten_minutes: {}", two_hours_ten_minutes);
    println!("std_duration: {:?}", std_duration);
    let from_duration = Duration::from_secs(3600);
    let chrono_duration = TimeDelta::from_std(from_duration).unwrap();
    println!("from_duration: {:?}", from_duration);
    println!("chrono_duration: {}", chrono_duration);
```
自己创建一个时间段实际应用中应该是比较少，比较多的是两个时间点相减获得一个时间段。

### DataTime -- 日期时间

DateTime结构体表示一个时区的日期时间，其是时区相关的。DateTime是相对于TimeDelta的"时间点"的概念,代表的是某一刻的时间。 日期限制在大约+/- 262,000年。

DateTime<Tz>结构体是泛型结构体，其中泛型Tz代表时区，是实现了TimeZone trait的类型。TimeZone trait 有三种实现：

* Utc : UTC 时区。当您不需要本地时间时，这是最有效的时区。 --- `DateTime<Utc>`
* Local : 系统本地时区 --- `DateTime<Local>`
* FixedOffset : 任意的固定时区,例如 UTC+09:00 或者 UTC-10:30 。这通常是由解析的文本日期和时间导致的。由于它存储的信息最多，并且不依赖于系统环境，因此您需要将其他时区规范化为此类型。 --- `DateTime<FixedOffset>`

DateTime对象是时区感知的，DateTime对象必须从TimeZone对象中构造，其结构体本身不存在相应的构造方法。所以首先要了解TimeZone对象的相关方法。
Utc/Local都有一个now方法，可以返回一个DateTime<Utc>/Date<Local>，后者包含了相对于Utc时区的偏移量。这是创建当前日期时间的最佳方法。
FixedOffset代表的是有相对于Utc时区固定偏移量的时区，偏移量的范围是[UTC-23:59:59,UTC+23:59:59].方法east_opt和west_opt可以创建特定的时区。
FixedOffset没有now方法获取该时区的当前日期时间，但是可以使用DateTime定义的方法with_timezone将DateTime<Utc>和DateTime<Local>对象转换成对应时区的日期时间。FixedOffset实现了Copy, Clone, Debug, Display, Eq, PartialEq, Add, Sub等trait，另外还实现了FromStr trait,因此可以将字符串转换成时区。

#### 构建特定时区

```rust
    let tz_east_6 = FixedOffset::east_opt(6 * 3600).expect("Out-of-bounds secs");
    println!("tz_east_6: {}", tz_east_6);

    let tz_west_10 = FixedOffset::west_opt(10 * 3600).expect("Out-of-bouds secs");
    println!("tz_west_6: {}", tz_west_10);

    let tz = FixedOffset::from_str("+0930").unwrap();
    println!("tz: {:?}", tz);

    let tz2 = FixedOffset::from_str("-1000").unwrap();
    println!("tz2: {}", tz2);
```

#### Timezone trait提供的创建DateTime的方法

TimeZone trait提供了很多创建日期时间的方法

* 从Unix时间戳中构建日期时间
* 从年月日和时间组件以及特定时区构建日期时间

```rust
    println!("{}", i64::MAX);
    let tz_local = FixedOffset::from_str("+0800").expect("out-of-bouds secs"); // 构建东八区时区

    // 从Unix时间戳中构建日期时间
    let dt_timestamp = tz_local.timestamp(i32::MAX as i64, 0); // deprecated 已经被弃用了
    println!("dt_timestamp: {}", dt_timestamp);
    let dt_timetamp_opt = tz_local.timestamp_opt(3000000, 0).unwrap();
    println!("dt: {}", dt_timetamp_opt);

    let dt_timestamp_nanos = tz_local.timestamp_nanos(i64::MAX); // 永远不会失败，不会out of bounds
    println!("dt_timestamp_nanos: {}", dt_timestamp_nanos);

    let dt_timestamp_micros = tz_local
        .timestamp_micros(100 * 365 * 24 * 60 * 60 * 1000 * 1000)
        .unwrap(); // 1970年1月1日开始大约100年
    println!("dt_timestamp_micros: {:?}", dt_timestamp_micros);
 
    let dt_timestamp_millis = tz_local.timestamp_millis(100000*365*24*60*60*1000);
    println!("dt_timestamp_mills: {}",dt_timestamp_millis);

    let dt_timestamp_millis_opt = tz_local.timestamp_millis_opt(100000*365*24*60*60*1000);
    println!("dt_timestamp_millis_opt: {:?}",dt_timestamp_millis_opt);
    // 从年月日和时分秒等组件和时区构建日期时间
    let dt = tz_local.with_ymd_and_hms(262142, 12, 31, 12, 30, 50);
    println!("dt: {:?}", dt);
```

#### 获取特定时区的当前日期时间

```rust
    // 2.1 从时区Timezone对象创建当前的日期时间
    // 2.1.1 获取当前Utc时区的当前日期时间
    let dt_utc = Utc::now();
    println!("dt_utc: {}", dt_utc);

    // 2.1.2 获取当前Local时区的当前日期时间
    let dt_local = Local::now();
    println!("dt_local: {}", dt_local);

    // 2.1.3 获取特定时区的当前日期时间

    let fixed_offset = FixedOffset::east_opt(6 * 3600).unwrap(); // 获取到东六区的时区
    let dt_fixed_offset = dt_utc.with_timezone(&fixed_offset); // 将当前日期时间转换成该时区的日期时间
    println!("dt_fixed_offset: {}",dt_fixed_offset);
```

以上知道了如何创建时区和从时区中创建DateTime对象，下面熟悉一下DateTime对象用法。

#### 通过加减一定的时间段计算出一个新的日期时间DateTime对象

```rust
    // 创建一个DateTime对象
    let dt = Local.with_ymd_and_hms(2024, 8, 23, 8, 30, 50).unwrap();
    println!("dt: {}", dt);

    // 对DateTime对象加减特定的时间段生成一个新的DateTime对象 -> Option<DateTime>
    let after_5days = dt.checked_add_days(Days::new(5)).unwrap();
    println!("after_5days: {}", after_5days);

    let after_1month = dt.checked_add_months(Months::new(1)).unwrap();
    println!("after_1month: {}", after_1month);

    let after_2days = dt.checked_add_signed(TimeDelta::days(2)).unwrap();
    println!("after_1day: {}", after_2days);
    let after_2weeks = dt.checked_add_signed(TimeDelta::weeks(1)).unwrap();
    println!("after_2weeks: {}", after_2weeks);

    let before_2days = dt.checked_sub_days(Days::new(2)).unwrap();
    println!("before_2days: {}", before_2days);

    let before_2months = dt.checked_sub_months(Months::new(2)).unwrap();
    println!("before_2months: {}", before_2months);

    let before_2weeks = dt.checked_sub_signed(TimeDelta::weeks(2)).unwrap();
    println!("before_2weeks: {}", before_2weeks);
```

#### DateTime对象相互转换

```rust

    // 将DateTime<Tz:TimeZone>类型转换成DateTime<FixedOffset>
    // 比如将DateTime<Utc>, DateTime<Local> .etc => DateTime<FixedOffset>
    let dt_fixed_offset = dt.fixed_offset();
    println!("dt_fixed_offset: {}", dt_fixed_offset);

    // 将DateTime<Tz> 转化成DateTime<Utc>
    let dt_utc = dt.to_utc();
    println!("dt_utc: {}", dt_utc);

```

#### 转换成无时区信息的NaiveDate/NaiveTime/NaiveDateTime

```rust
// 转换成无时区信息的NaiveDate/NaiveTime/NaiveDateTime
    let naive_date = dt.date_naive();
    println!("naive_date: {}", naive_date);
    let naive_time = dt.time();
    println!("naive_time: {}", naive_time);
    let naive_dt_local = dt.naive_local();
    println!("naive_dt_local: {}", naive_dt_local);
    let naive_dt_utc = dt.naive_utc();
    println!("naive_dt_utc: {}", naive_dt_utc);
```

#### 获取时区和偏移量

```rust 
    // 获取时区和偏移量
    let tz = dt.timezone();
    println!("tz: {:?}", tz);
    let offset = dt.offset();
    println!("offset: {:?}", offset);
```

#### 格式化日期时间

```rust 
// 格式化日期时间
    let delayed_format = dt.format("%Y-%m-%d %H:%M:%S %z");
    println!("delayed_format: {}", delayed_format);
    let dt_str = format!("{}", delayed_format);
    println!("dt_str: {}", dt_str);

    let dt_format = dt.format_localized("%F", chrono::Locale::zh_CN);
    let dt_str = format!("{}", dt_format);
    println!("dt_str: {}", dt_str);
```

#### 将字符串解析为DateTime对象

```rust
    // 将字符串解析为DateTime对象
    // parse_from_str传入的字符串必须包含时区信息
    let dt1 = DateTime::parse_from_str("2024-08-23 10:54:20 -0000", "%F %H:%M:%S %z").unwrap();
    println!("dt1: {}", dt1);

    let (dt2, s) =
        DateTime::parse_and_remainder("2024-08-23 10:54:20 -0000Hello,world!", "%F %H:%M:%S %z")
            .unwrap();
    println!("dt2: {}", dt2);
    println!("remainder: {}", s);

    let dt3 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap();
    println!("dt3: {}", dt3);

    let dt4 = DateTime::parse_from_rfc2822("Wed, 18 Feb 2015 23:16:09 GMT").unwrap();
    println!("dt4: {}", dt4);
```

DateTime对象实现了Clone, Copy, Debug, Display, Default, Eq, PartialEq, Ord, PartialOrd等trait，因此可以克隆、复制、打印显示、生成默认值和比较。

DateTime对象还实现了`Add<TimeDelta>`和Sub`<TimeDelta>`，因此可以加一个时间段生成一个新的日期时间。它还实现了`Sub<DateTime>`，可以和另一个日期时间相减，计算出间隔的时间段TimeDelta对象。

## jsonwebtoken -- 强类型的方式创建和解析jwt

### JSON Web Token

JWT是一种开放的行业标准 （RFC 7519）,定义了一种紧凑且独立的在各方之间安全传递信息的方式。这些信息以JSON对象的形式传递。
这些信息经过了数字签名，因此可以验证。可以使用密钥（使用 HMAC 算法）或使用 RSA 或 ECDSA 的公钥/私钥对对 JWT 进行签名。
签名的令牌有以下特点：

* 签名的令牌可以验证其包含的声明（claims）的完整性
* 令牌是加密的，可以对其他各方隐藏，保证安全。

当使用公钥/私钥对对令牌进行签名时，签名还证明只有持有私钥的一方才是签署私钥的一方。

#### JWT的应用

* 授权：这是使用 JWT 的最常见方案。用户登录后，每个后续请求都将包含 JWT，允许用户访问该令牌允许的路由、服务和资源。单点登录是当今广泛使用 JWT 的一项功能，因为它的开销小，并且能够在不同域中轻松使用。
* 信息交换：JSON Web 令牌是在各方之间安全传输信息的好方法。由于可以对 JWT 进行签名（例如，使用公钥/私钥对），因此您可以确定发送者就是他们所说的人。此外，由于签名是使用标头和有效负载计算的，因此您还可以验证内容是否未被篡改。

#### JWT的结构

在其紧凑的形式中，JSON Web 令牌由三个部分组成，由点 （.） 分隔三个组分。三个组分是：

* Header 头部
* payload 有效荷载
* signature 签名

JWT的形式：xxxxx.yyyyy.zzzzz

##### Header 头部

标头通常由两部分组成：令牌的类型（即 JWT）和正在使用的签名算法，例如 HMAC SHA256 或 RSA。

```plain
{
    "alg": "HS256",
    "typ": "JWT"
}
```
然后，将此 JSON 进行 Base64Url 编码，以构成 JWT 的第一部分。

##### Payload 有效载荷

令牌的第二部分是有效负载，其中包含声明（claims）。声明是关于实体（通常是用户）和其他数据的陈述。有三种类型的声明：注册声明、公共声明和私人声明。

* 已注册的声明：这些是一组预定义的声明，这些声明不是强制性的，但建议使用，以提供一组有用的、可互操作的声明。其中一些是：iss（发行人）、exp（到期时间）、sub（主题）、aud（受众）等。
* 公共声明：使用 JWT 的人可以随意定义这些声明。但为了避免冲突，它们应该在 IANA JSON Web 令牌注册表中定义，或者定义为包含防冲突命名空间的URI。
* 私人声明：这些是自定义声明，用于在同意使用它们的各方之间共享信息，既不是注册声明，也不是公开声明。

有效负载示例可能是：

```plain
{
  "sub": "1234567890",
  "name": "John Doe",
  "admin": true
}
```

然后，对有效负载进行 Base64Url 编码，以形成 JSON Web 令牌的第二部分。

##### signature 签名

要创建签名部分，您必须获取编码的标头、编码的有效负载、密钥、标头中指定的算法，并对其进行签名。

例如，如果要使用 HMAC SHA256 算法，将按以下方式创建签名：
```shell
HMACSHA256(
  base64UrlEncode(header) + "." +
  base64UrlEncode(payload),
  secret)
```

签名用于验证消息在传输过程中未被更改，并且，对于使用私钥签名的令牌，它还可以验证 JWT 的发送者是否是它所说的那个人。

##### 组合三个部分形成jwt

最终的jwt就是三个用点分隔Base64-URL字符串。可以在 HTML 和 HTTP 环境中轻松传递，同时与基于 XML 的标准（如 SAML）相比更紧凑。

下面显示了一个 JWT，它编码了前一个标头和有效负载，并且它使用密钥进行签名。

```plain
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ3YW5nemhpZmVpeGlhbmdAcXEuY29tIiwiY29tcGFueSI6Ind6ZiIsImV4cCI6NjA0ODAwfQ.dCxxaNcAHdQhBJhvC0R0uknwUJBiE5BoNu5FYs_wZQY
```

#### JWT工作原理

在身份验证中，当用户使用其凭据成功登录时，将返回 JSON Web 令牌。由于令牌是凭据，因此必须非常小心地防止安全问题。通常，令牌的保留时间不应超过所需时间。

由于缺乏安全性，您也不应将敏感会话数据存储在浏览器存储中。

每当用户想要访问受保护的路由或资源时，用户代理都应发送 JWT，通常在 Authorization 标头中使用 Bearer 架构。标头的内容应如下所示：

```plain
Authorization: Bearer <token>
```

在某些情况下，这可以是无状态授权机制。服务器的受保护路由将在 Authorization 标头中检查有效的 JWT，如果存在，则将允许用户访问受保护的资源。如果 JWT 包含必要的数据，则可能会减少查询数据库以执行某些操作的需求，尽管情况可能并非总是如此。

请注意，如果通过 HTTP 标头发送 JWT 令牌，则应尽量防止它们变得太大。某些服务器不接受超过 8 KB 的标头。如果您尝试在 JWT 令牌中嵌入太多信息，例如包含所有用户的权限，您可能需要替代解决方案，例如 Auth0 细粒度授权。

如果令牌是在 Authorization 标头中发送的，则跨域资源共享 （CORS） 不会成为问题，因为它不使用 Cookie。

### jsonwebtoken crate

