

支付宝api文档：https://opendocs.alipay.com/apis/

签名方法为 RSA2，采用支付宝提供的 [RSA签名&验签工具](https://opendocs.alipay.com/open/291/105971) 生成秘钥时，秘钥的格式必须为 PKCS1，秘钥长度推荐 2048。所以在支付宝管理后台请注意配置 RSA2(SHA256)密钥。


### Usage:  

```toml  

[dependencies]  
alipay-rs = {git = "https://github.com/chenhuaiyuan/alipay-rs"}  
# 已在原先的alipay-rs库中删除AlipayParam宏，需要添加struct-map库来实现AlipayParam宏，如果未使用到AlipayParam宏可以不添加   
struct-map = {git = "https://github.com/chenhuaiyuan/struct-map"}  

# or

alipay-rs = "0.2"  
struct-map = "0.1"

```

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
        include_str!("../私钥.txt"),
        Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
        Some(include_str!("../alipayRootCert.crt"))
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
