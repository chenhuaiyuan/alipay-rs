

支付宝api文档：https://opendocs.alipay.com/apis/

签名方法为 RSA2，采用支付宝提供的 [RSA签名&验签工具](https://opendocs.alipay.com/open/291/105971) 生成秘钥时，秘钥的格式必须为 PKCS1，秘钥长度推荐 2048。所以在支付宝管理后台请注意配置 RSA2(SHA256)密钥。


### example  

以单笔转账接口为例：   

```rust

...

// 接口参数
#[derive(Serialize, Debug)]
struct Transfer {
    out_biz_no: String,
    trans_amount: String,
    product_code: String,
    biz_scene: String,
    payee_info: PayeeInfo,
}
#[derive(Serialize, Debug)]
struct PayeeInfo {
    identity: String,
    identity_type: String,
    name: String,
}

async fn fund_transfer() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let client = alipay_rs::Client::new(
        "20210xxxxxxxxxxx",
        "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
        Some("appCertPublicKey_20210xxxxxxxxxxx.crt"),
        Some("alipayRootCert.crt")
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}

async fn neo_fund_transfer() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let client = alipay_rs::Client::neo(
        "20210xxxxxxxxxxx",
        "私钥.txt",
        Some("appCertPublicKey_20210xxxxxxxxxxx.crt"),
        Some("alipayRootCert.crt")
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
```  
支付宝的所有接口都可以使用client.post函数访问，如果接口没有参数，可以使用client.no_param_post函数。  
默认的公共参数包含：app_id，charset，sign_type，format，version，method，timestamp，sign，如果想修改参数值，可以通过client.set_public_params函数设置。  
```rust

...

// 需要修改或添加公共参数必须添加AlipayParam宏
#[derive(AlipayParam)]
struct PublicParams {
    app_id: String,
    method: Option<String>,
    charset: String,
    sign_type: String,
    sign: Option<String>,
    timestamp: Option<String>,
    version: String,
}

...

let public_params = PublicParams {
    app_id: "20210xxxxxxxxxxx".to_owned(),
    method: None,
    charset: "utf-8".to_owned(),
    sign_type: "RSA2".to_owned(),
    sign: None,
    timestamp: None,
    version: "1.0".to_owned(),
};
// 公共参数值为None或参数名不存在于公共参数会被过滤掉
client.set_public_params(public_params);

...

```  

如果需要添加公共参数，可通过client.add_public_params函数设置, 具体可以看看example中的image_upload。   
```rust   
...

#[derive(AlipayParam)]
struct ImageUpload {
    image_type: String,
    image_name: String,
}

...

let image = ImageUpload {
    image_type: "png".to_owned(),
    image_name: "test".to_owned(),
};

...

client.add_public_params(image);

```   

alipay api有图片视频等资源上传的接口，可以通过post_file接口进行资源上传   
```rust
#[derive(AlipayParam)]
struct Image {
    image_type: String,
    image_name: String,
}
let file = std::fs::read("./test.png").unwrap();
let image = Image {
    image_type: "png".to_owned(),
    image_name: "test".to_owned(),
};
let mut client = ...;
client.add_public_params(image);
// post_file参数：
// method 接口名称
// key 文件参数名
// file_name 文件名
// file_content 文件内容
let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
println!("{:?}", data);
```
