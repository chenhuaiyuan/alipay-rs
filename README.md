# alipay-rs

支付宝api文档: <https://opendocs.alipay.com/apis/>

签名方法为 RSA2，采用支付宝提供的 [RSA签名&验签工具](https://opendocs.alipay.com/open/291/105971) 生成秘钥时，秘钥的格式必须为 PKCS1，秘钥长度推荐 2048。所以在支付宝管理后台请注意配置 RSA2(SHA256)密钥。

这是一个简单的alipay SDK，只需要创建client，然后通过client的post方法请求Alipay api即可。

# Note

```text

新版本与之前版本不兼容，但之前代码只需少量修改就可继续使用。

```

```rust
// 老代码，v0.3版本之前写法
let client = alipay_rs::Client::new(
    "20210xxxxxxxxxxx",
    include_str!("../私钥.txt"),
    Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    Some(include_str!("../alipayRootCert.crt"))
);

// 新代码，v0.3版本之后写法
let client = alipay_rs::Client::builder()
    .app_id("20210xxxxxxxxxxx")
    .public_key(include_str!("../公钥.txt"))
    .private_key(include_str!("../私钥.txt"))
    .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
    .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
    .finish();

// or

let client = alipay_rs::Client::new(
    "20210xxxxxxxxxxx",
    include_str!("../公钥.txt"),    // 新增
    include_str!("../私钥.txt"),
    Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    Some(include_str!("../alipayRootCert.crt"))
);
```

## Usage

```toml

[dependencies]
alipay-rs = "0.3"
# 如果不会修改公共参数，可以不添加以下依赖
alipay_params = "0.1"

```

## example

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
    let client = alipay_rs::Client::builder()
        .app_id("20210xxxxxxxxxxx")
        .public_key(include_str!("../公钥.txt"))
        .private_key(include_str!("../私钥.txt"))
        .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
        .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
        .finish();
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
```

支付宝的所有接口都可以使用client.post函数访问，如果接口没有参数，可以使用client.no_param_post函数。
默认的公共参数包含：app_id，charset，sign_type，format，version，method，timestamp，sign，如果想修改或添加参数值，可以通过client.set_public_params函数设置。

```rust

...

// 可以通过AlipayParam宏来定义需要添加或修改的公共参数
#[derive(AlipayParam)]
struct PublicParams {
    app_id: String,
    charset: String,
    sign_type: String,
    version: String,
}

...

let public_params = PublicParams {
    app_id: "20210xxxxxxxxxxx".to_owned(),
    charset: "utf-8".to_owned(),
    sign_type: "RSA2".to_owned(),
    version: "1.0".to_owned(),
};

// 也可以通过vec, hashmap, array, tuple来设置公共参数
let public_params = ("app_id", "20210xxxxxxxxxxx");
let public_params = [("image_type", "png"), ("image_name", "test")];
let public_params = vec![("image_type", "png"), ("image_name", "test")];
let public_params = HashMap::from([("image_type", "png"), ("image_name", "test")]);

let mut client_with_params = client.set_public_params(public_params);

...

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
let client = ...;
let mut client_with_params = client.set_public_params(image);
// post_file参数：
// method 接口名称
// key 文件参数名
// file_name 文件名
// file_content 文件内容
let data:serde_json::Value = client_with_params.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
println!("{:?}", data);
```

## mutlithreading example

```rust

......

async fn ref_query(client: &alipay_rs::Client) {
    let query = QueryParam {
        operation: "ITEM_PAGEQUERY".to_owned(),
        page_num: 1,
        page_size: 10,
        item_id_list: None,
    };

    let data:serde_json::Value = client
        .post("alipay.open.mini.item.page.query", query)
        .await.unwrap();
    println!("{:?}", data);
}

async fn ref_fund_transfer(client: &alipay_rs::Client) {
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
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}

......

let client = alipay_rs::Client::builder()
.app_id("20210xxxxxxxxxxx")
.public_key(include_str!("../公钥.txt"))
.private_key(include_str!("../私钥.txt"))
.app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
.alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
.finish();

let cli = Arc::new(client);
let cli_clone = cli.clone();
tokio::spawn(async move {
    ref_query(&cli_clone).await;
}).await.unwrap();
tokio::spawn(async move {
    ref_fund_transfer(&cli.clone()).await;
}).await.unwrap();
```
