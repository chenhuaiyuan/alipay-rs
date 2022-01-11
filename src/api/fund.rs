use super::Fund;
use crate::error::AlipayResult;
use serde::{de::DeserializeOwned, Serialize};

impl Fund {
    /// 单笔转账接口
    ///
    /// Example:
    /// ```rust
    /// use serde::Serialize;
    /// use chrono::Local;
    /// use alipay_rs::param::{AlipayParam, FieldValue};
    ///
    /// #[derive(Serialize, Debug)]
    /// struct Transfer {
    ///     out_biz_no: String,
    ///     trans_amount: String,
    ///     product_code: String,
    ///     biz_scene: String,
    ///     payee_info: PayeeInfo,
    /// }
    /// #[derive(Serialize, Debug)]
    /// struct PayeeInfo {
    ///     identity: String,
    ///     identity_type: String,
    ///     name: String,
    /// }
    /// async fn fund_transfer() {
    ///     let transfer = Transfer {
    ///         out_biz_no: format!("{}", Local::now().timestamp()),
    ///         trans_amount: String::from("0.1"),
    ///         product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
    ///         biz_scene: String::from("DIRECT_TRANSFER"),
    ///         payee_info: PayeeInfo {
    ///             identity: String::from("343938938@qq.com"),
    ///             identity_type: String::from("ALIPAY_LOGON_ID"),
    ///             name: String::from("陈怀远"),
    ///         },
    ///     };
    ///     let client = alipay_rs::Client::new(
    ///         "20210xxxxxxxxxxx",
    ///         include_str!("../私钥.txt"),
    ///         Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    ///         Some(include_str!("../alipayRootCert.crt"))
    ///     );
    ///     let fund = alipay_rs::api::Fund::new(client);
    ///     let data: serde_json::Value = fund.fund_trans_uni_transfer(client, transfer).await.unwrap();
    ///     println!("{:?}", data);
    /// }
    /// ```
    pub async fn fund_trans_uni_transfer<T: Serialize, R: DeserializeOwned>(
        self,
        params: T,
    ) -> AlipayResult<R> {
        let data: R = self
            .client
            .post("alipay.fund.trans.uni.transfer", params)
            .await?;
        Ok(data)
    }
}
