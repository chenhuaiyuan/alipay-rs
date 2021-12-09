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
    ///         "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
    ///         Some("appCertPublicKey_20210xxxxxxxxxxx.crt"),
    ///         Some("alipayRootCert.crt")
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
