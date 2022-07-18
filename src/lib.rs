//! 对alipay api的简单封装
//!
//! # Usage:
//! ```toml
//! [dependencies]
//! alipay-rs = {git = "https://github.com/chenhuaiyuan/alipay-rs"}
//!
//! or
//!
//! alipay-rs = "0.3"
//!
//! ```
//!
//! # Example:
//! ```rust
//! // 默认的公共参数只包含了最基础的，如果需要增加公共参数，可用通过set_public_params函数实现
//! // 默认的公共参数包含：app_id，charset，sign_type，format，version，method，timestamp，sign
//! // 通过set_public_params设置公共参数，如果参数值为None会自动过滤，重复的参数后面的值会覆盖前面的值
//! // 下面是单笔转账的几种示例
//! use serde::Serialize;
//! use chrono::{Local};
//! use alipay_rs::AlipayParam;
//!
//! // 单笔转账接口需要的参数
//! #[derive(Serialize, Debug)]
//! struct Transfer {
//!     out_biz_no: String,
//!     trans_amount: String,
//!     product_code: String,
//!     biz_scene: String,
//!     payee_info: PayeeInfo,
//! }
//! #[derive(Serialize, Debug)]
//! struct PayeeInfo {
//!     identity: String,
//!     identity_type: String,
//!     name: String,
//! }
//!
//! // 通过post方法访问单笔转账接口
//! async fn naive_fund_transfer() {
//!     let transfer = Transfer {
//!         out_biz_no: format!("{}", Local::now().timestamp()),
//!         trans_amount: String::from("0.1"),
//!         product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
//!         biz_scene: String::from("DIRECT_TRANSFER"),
//!         payee_info: PayeeInfo {
//!             identity: String::from("343938938@qq.com"),
//!             identity_type: String::from("ALIPAY_LOGON_ID"),
//!             name: String::from("陈怀远"),
//!         },
//!     };
//!     let config = alipay_rs::Config::builder()
//!        .app_id("20210xxxxxxxxxxx")
//!        .public_key(include_str!("../公钥.txt"))
//!        .private_key(include_str!("../私钥.txt"))
//!        .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
//!        .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
//!        .finish();
//!     let mut client = config.get_client();
//!     let data:serde_json::Value = client
//!         .post("alipay.fund.trans.uni.transfer", transfer)
//!         .await.unwrap();
//!     println!("{:?}", data);
//! }
//!
//!
//! // 公共参数
//! #[derive(AlipayParam)]
//! struct PublicParams {
//!     app_id: String,
//!     method: Option<String>,
//!     charset: String,
//!     sign_type: String,
//!     sign: Option<String>,
//!     timestamp: Option<String>,
//!     version: String,
//! }
//! // 修改公共参数来访问单笔转账接口
//! async fn fund_transfer_from_public_params() {
//!     let transfer = Transfer {
//!         out_biz_no: format!("{}", Local::now().timestamp()),
//!         trans_amount: String::from("0.1"),
//!         product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
//!         biz_scene: String::from("DIRECT_TRANSFER"),
//!         payee_info: PayeeInfo {
//!             identity: String::from("343938938@qq.com"),
//!             identity_type: String::from("ALIPAY_LOGON_ID"),
//!             name: String::from("陈怀远"),
//!         },
//!     };
//!     let config = alipay_rs::Config::builder()
//!        .app_id("20210xxxxxxxxxxx")
//!        .public_key(include_str!("../公钥.txt"))
//!        .private_key(include_str!("../私钥.txt"))
//!        .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
//!        .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
//!        .finish();
//!     let mut client = config.get_client();
//!     let public_params = PublicParams {
//!         app_id: "20210xxxxxxxxxxx".to_owned(),
//!         method: None,
//!         charset: "utf-8".to_owned(),
//!         sign_type: "RSA2".to_owned(),
//!         sign: None,
//!         timestamp: None,
//!         version: "1.0".to_owned(),
//!     };
//!     client.set_public_params(public_params);
//!     let data:serde_json::Value = client
//!         .post("alipay.fund.trans.uni.transfer", transfer)
//!         .await.unwrap();
//!     println!("{:?}", data);
//! }
//!
//! // 上传图片文件
//! async fn image_upload() {
//! let file = std::fs::read("./test.png").unwrap();
//! let image = [("image_type", "png"), ("image_name", "test")];
//! let config = alipay_rs::Config::builder()
//!    .app_id("20210xxxxxxxxxxx")
//!    .public_key(include_str!("../公钥.txt"))
//!    .private_key(include_str!("../私钥.txt"))
//!    .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
//!    .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
//!    .finish();
//! let mut client = config.get_client();
//! client.set_public_params(image);
//!
//! let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
//! println!("{:?}", data);
//! }
//! #[tokio::main]
//! async fn main() {
//!     // naive_fund_transfer().await;
//!     // fund_transfer().await;
//!     fund_transfer_from_public_params().await;
//! }
//! ```
//! # Example2:
//! ```rust
//! use alipay_rs::AlipayParam;
//! use chrono::Local;
//! use serde::Serialize;
//! use std::collections::HashMap;
//!
//! #[derive(Serialize, Debug)]
//! struct Transfer {
//!     out_biz_no: String,
//!     trans_amount: String,
//!     product_code: String,
//!     biz_scene: String,
//!     payee_info: PayeeInfo,
//! }
//! #[derive(Serialize, Debug)]
//! struct PayeeInfo {
//!     identity: String,
//!     identity_type: String,
//!     name: String,
//! }
//!
//! #[derive(Debug, Serialize)]
//! struct QueryParam {
//!     operation: String,
//!     page_num: i32,
//!     page_size: i32,
//!     item_id_list: Option<String>
//! }
//!
//! async fn ref_query(client: &mut alipay_rs::Client) {
//!     let query = QueryParam {
//!         operation: "ITEM_PAGEQUERY".to_owned(),
//!         page_num: 1,
//!         page_size: 10,
//!         item_id_list: None,
//!     };
//!
//!     let data:serde_json::Value = client
//!         .post("alipay.open.mini.item.page.query", query)
//!         .await.unwrap();
//!     println!("{:?}", data);
//! }
//!
//! async fn ref_fund_transfer(client: &mut alipay_rs::Client) {
//!     let transfer = Transfer {
//!         out_biz_no: format!("{}", Local::now().timestamp()),
//!         trans_amount: String::from("0.1"),
//!         product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
//!         biz_scene: String::from("DIRECT_TRANSFER"),
//!         payee_info: PayeeInfo {
//!             identity: String::from("343938938@qq.com"),
//!             identity_type: String::from("ALIPAY_LOGON_ID"),
//!             name: String::from("陈怀远"),
//!         },
//!     };
//!     let data:serde_json::Value = client
//!         .post("alipay.fund.trans.uni.transfer", transfer)
//!         .await.unwrap();
//!     println!("{:?}", data);
//! }
//! #[tokio::main]
//! async fn main() {
//!
//!     let config = alipay_rs::Config::builder()
//!        .app_id("20210xxxxxxxxxxx")
//!        .public_key(include_str!("../公钥.txt"))
//!        .private_key(include_str!("../私钥.txt"))
//!        .app_cert_sn(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt"))
//!        .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
//!        .finish();
//!     let mut client = config.get_client();
//!
//!     ref_query(&client).await;
//!     ref_fund_transfer(&client).await;
//!
//!     // 多线程调用
//!     let conf = Arc::new(config);
//!     let conf_clone = conf.clone();
//!     tokio::spawn(async move {
//!         let mut client = conf_clone.get_client();
//!         ref_query(&mut client).await;
//!     }).await.unwrap();
//!     tokio::spawn(async move {
//!         let mut client = conf.clone().get_client();
//!         ref_fund_transfer(&mut client).await;
//!     }).await.unwrap();
//! }
//! ```

mod alipay;
mod app_cert_client;
mod config;
mod util;

pub use alipay::Sign;
pub use config::Config;
pub use config::ConfigBuilder;
pub mod error;
pub use alipay::Client;
pub use alipay_params::{AlipayParam, PublicParams};
