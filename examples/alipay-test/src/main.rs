use serde::Serialize;
use chrono::{Local};
use alipay_rs::{AlipayParam, FieldValue};

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
async fn naive_fund_transfer() {
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
    let mut client = alipay_rs::Client::new(
        "20210xxxxxxxxxxx",
        include_str!("../公钥.txt"),
        include_str!("../私钥.txt"),
        Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
        Some(include_str!("../alipayRootCert.crt"))
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
#[derive(Serialize, Debug)]
struct StationQuery {
    city_code: String
}
async fn naive_station_query() {
    let station_query = StationQuery {
        city_code: String::from("330100")
    };
    let mut client = alipay_rs::Client::new(
        "20210xxxxxxxxxxx",
        include_str!("../公钥.txt"),
        include_str!("../私钥.txt"),
        None,
        None,
    );
    let data:serde_json::Value = client
        .post("alipay.commerce.cityfacilitator.station.query", station_query)
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
    let mut client = alipay_rs::Client::neo(
        "20210xxxxxxxxxxx",
        "公钥.txt",
        "私钥.txt",
        Some("appCertPublicKey_20210xxxxxxxxxxx.crt"),
        Some("alipayRootCert.crt")
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
// 已放弃
// async fn fund_transfer() {
//     let transfer = Transfer {
//         out_biz_no: format!("{}", Local::now().timestamp()),
//         trans_amount: String::from("0.1"),
//         product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
//         biz_scene: String::from("DIRECT_TRANSFER"),
//         payee_info: PayeeInfo {
//             identity: String::from("343938938@qq.com"),
//             identity_type: String::from("ALIPAY_LOGON_ID"),
//             name: String::from("陈怀远"),
//         },
//     };
//     let client = alipay_rs::Client::new(
//         "20210xxxxxxxxxxx",
//         include_str!("../私钥.txt"),
//         Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
//         Some(include_str!("../alipayRootCert.crt"))
//     );
//     // 已放弃此调用方法
//     let api = alipay_rs::api::Fund::new(client);
//     let data: serde_json::Value = api.fund_trans_uni_transfer(transfer).await.unwrap();
//     println!("{:?}", data);
// }

#[derive(AlipayParam)]
struct PublicParams {
    app_id: String,
    method: Option<String>,
    charset: String,
    sign_type: String,
    sign: Option<String>,
    timestamp: Option<String>,
    version: String,
    biz_content: Option<String>,
}
async fn fund_transfer_from_public_params() {
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
    let mut client = alipay_rs::Client::new(
        "20210xxxxxxxxxxx",
        include_str!("../公钥.txt"),
        include_str!("../私钥.txt"),
        Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
        Some(include_str!("../alipayRootCert.crt"))
    );
    let public_params = PublicParams {
        app_id: "20210xxxxxxxxxxx".to_owned(),
        method: None,
        charset: "utf-8".to_owned(),
        sign_type: "RSA2".to_owned(),
        sign: None,
        timestamp: None,
        version: "1.0".to_owned(),
        biz_content: None,
    };
    client.set_public_params(public_params);
    // 已放弃此调用方法
    // let api = alipay_rs::api::Fund::new(client);
    // let data: serde_json::Value = api.fund_trans_uni_transfer(transfer).await.unwrap();

    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}

#[derive(AlipayParam)]
struct ImageUpload {
    image_type: String,
    image_name: String,
}

async fn image_upload() {
    let file = std::fs::read("./test.png").unwrap();
    let image = ImageUpload {
        image_type: "png".to_owned(),
        image_name: "test".to_owned(),
    };
    let mut client = alipay_rs::Client::new(
        "20210xxxxxxxxxxx",
        include_str!("../公钥.txt"),
        include_str!("../私钥.txt"),
        Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
        Some(include_str!("../alipayRootCert.crt"))
    );
    client.add_public_params(image);

    let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
    println!("{:?}", data);
}

#[derive(Debug, Serialize)]
struct QueryParam {
    operation: String,
    page_num: i32,
    page_size: i32,
    item_id_list: Option<String>
}
async fn ref_query(client: &mut alipay_rs::Client) {
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

async fn ref_fund_transfer(client: &mut alipay_rs::Client) {
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

async fn sign_verify(client: &mut alipay_rs::Client) {
    let str = "alipay_root_cert_sn=687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6&app_cert_sn=ba7c22914aaacc8e923e5af8befccd58&app_id=2021002199679230&biz_content={\"operation\":\"ITEM_PAGEQUERY\",\"page_num\":1,\"page_size\":10,\"item_id_list\":null}&charset=utf-8&format=json&method=alipay.open.mini.item.page.query&sign_type=RSA2&timestamp=2022-07-17 00:27:58&version=1.0";
    let sign = client.sign(str).unwrap();
    let res = client.verify(str, &sign);
    println!("{:?}", res);
}

#[tokio::main]
async fn main() {
    // naive_fund_transfer().await;
    // fund_transfer().await;
    neo_fund_transfer().await;
    // fund_transfer_from_public_params().await;
    // image_upload().await;


    // let mut client = alipay_rs::Client::new(
    //     "20210xxxxxxxxxxx",
    //     include_str!("../公钥.txt"),
    //     include_str!("../私钥.txt"),
    //     Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    //     Some(include_str!("../alipayRootCert.crt"))
    // );

    // 单线程调用
    // ref_query(&client).await;
    // ref_fund_transfer(&client).await;


    // 多线程调用
    let config = alipay_rs::Config::builder()
    .app_id("2021002199679230")
    .public_key(include_str!("../公钥.txt"))
    .private_key(include_str!("../私钥.txt"))
    .app_cert_sn(include_str!("../appCertPublicKey_2021002199679230.crt"))
    .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
    .finish();
    let conf = Arc::new(config);
    let conf_clone = conf.clone();
    tokio::spawn(async move {
        let mut client = conf_clone.get_client();
        ref_query(&mut client).await;
    }).await.unwrap();
    tokio::spawn(async move {
        let mut client = conf.clone().get_client();
        ref_fund_transfer(&mut client).await;
    }).await.unwrap();
}
