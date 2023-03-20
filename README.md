# alipay-rs

支付宝api文档: <https://opendocs.alipay.com/apis/>

签名方法为 RSA2，采用支付宝提供的 [RSA签名&验签工具](https://opendocs.alipay.com/open/291/105971) 生成秘钥时，秘钥的格式必须为 PKCS1（现如今生成的秘钥可能不是PKCS1，需要先通过秘钥工具转换一下），秘钥长度推荐 2048。所以在支付宝管理后台请注意配置 RSA2(SHA256)密钥。

这是一个简单的alipay SDK，只需要创建client，然后通过client的post方法请求Alipay api即可。

## Usage

```toml

[dependencies]
alipay-rs = "0.4"
alipay_params = "0.1"

```

## example

以单笔转账接口为例：

```rust

...

// 接口参数
// 参数调用需要使用AlipayParams宏
#[derive(AlipayParams, Debug)]
struct Transfer {
    out_biz_no: String,
    trans_amount: String,
    product_code: String,
    biz_scene: String,
    payee_info: PayeeInfo,
}
#[derive(AlipayParams, Debug)]
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
        .await.unwrap().into_json().unwrap();
    println!("{:?}", data);
}
```

支付宝的所有接口都可以使用client.post函数访问，如果接口没有参数，可以使用client.no_param_post函数。
默认的公共参数包含：app_id，charset，sign_type，format，version，method，timestamp，sign，如果想修改或添加参数值，可以通过client.set_public_params函数设置。

```rust

...

// 可以通过AlipayParams宏来定义需要添加或修改的公共参数
#[derive(AlipayParams)]
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
#[derive(AlipayParams)]
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
let data:serde_json::Value = client_with_params.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap().into_json().unwrap();
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
        .await.unwrap().into_json().unwrap();
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
        .await.unwrap().into_json().unwrap();
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

## pc支付示例

pc端支付功能需要多做几步处理，比如qr_pay_mode等于0,1,3,4。
需要以iframe方式请求，而iframe所需要的form表单代码需要自行实现。
可以先通过generate_url_data来获取对应参数，如下：

```rust
let data = cli
        .generate_url_data("alipay.trade.page.pay", params)
        .unwrap();
```

然后通过数据拼接成form表单代码，下面是lua代码的实现：

```lua
local pay_with_params = pay:set_public_params({ notify_url = 'https://domain_name/v1/alipay/receive_notify' })
  local data = {
    ['out_trade_no'] = generate_order_no(),
    ['total_amount'] = params.total_amount,
    ['subject'] = 'test',
    ['product_code'] = 'FAST_INSTANT_TRADE_PAY',
    ['qr_pay_mode'] = 4,
    ['qrcode_width'] = 120,
  }
  local url_data = pay_with_params:generate_url_data('alipay.trade.page.pay', data)
  local html = '<form name=\'submit_form\' method=\'post\' action=\'https://openapi.alipay.com/gateway.do?charset=utf-8\'>\n'
  for k, v in pairs(url_data) do
    html = html .. '<input type=\'hidden\' name=\''
    html = html .. k
    html = html .. '\' value=\''
    html = html .. v
    html = html .. '\'>\n'
  end
  html = html .. '<input type=\'submit\' value=\'立即支付\' style=\'display:none\' >\n'
  html = html .. '</form>\n'
  html = html .. '<script>document.forms[0].submit();</script>'
```

生成form代码如下：

```html
<form name='submit_form' method='post' action='https://openapi.alipay.com/gateway.do?charset=utf-8'>\n<input type='hidden' name='timestamp' value='2022-12-22 05:34:57'>\n<input type='hidden' name='charset' value='utf-8'>\n<input type='hidden' name='format' value='json'>\n<input type='hidden' name='app_id' value='2021003172653113'>\n<input type='hidden' name='sign_type' value='RSA2'>\n<input type='hidden' name='version' value='1.0'>\n<input type='hidden' name='biz_content' value='{\"out_trade_no\":\"2022122305347606\",\"product_code\":\"FAST_INSTANT_TRADE_PAY\",\"qr_pay_mode\":4,\"qrcode_width\":120,\"subject\":\"test\",\"total_amount\":\"0.01\"}'>\n<input type='hidden' name='sign' value='UkxpmPyJc1YPVL8sZmc0dX2MO5XCY2GTENSe6JectGjDGilZiWHmxr2ibdKGZwV1N0a1yhu9m8itQ9zVWXvx82ODiCM7A8srp6YoMjXqgO2W8BpdcU5zhBNWj5xiolyY72dYGUATCJVpcwUWpPyRiF71wyGMYJ+x5pS21+S39jGIHmObEO+19tiI0meCcEMFC4DubsAZAOaAkgNCJ7eNnCglPRMCwxl79KSEc6gCLjeZpPS+nQTX/Pxtk2d4y2xHPnQqvSoADxx138w/Uw79OE6EHoBhoDIT9fp/gIanLE32hoel7pum8jjsF3ebk/X7WQCkwWkF6PGmHIIHAWPW8A=='>\n<input type='hidden' name='notify_url' value='https://domain_name/v1/alipay/receive_notify'>\n<input type='hidden' name='method' value='alipay.trade.page.pay'>\n<input type='submit' value='立即支付' style='display:none' >\n</form>\n<script>document.forms[0].submit();</script>
```

再将生成的form代码放入iframe中，代码如下：

```html
<!-- html为所生成form代码 -->
<iframe
    :srcdoc="html"
    frameborder="no"
    border="0"
    marginwidth="0"
    marginheight="0"
    scrolling="no"
    style="overflow: hidden"
    height="120"
    width="120"
/>
```
