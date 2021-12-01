use crate::app_cert_client;
use crate::error::AlipayResult;
use crate::param::{AlipayParam, FieldValue};
use crate::Client;
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::Signer,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

impl Client {
    pub fn new<S: Into<String>>(
        app_id: S,
        private_key: S,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Client {
        let mut params: HashMap<String, String> = HashMap::from([
            ("app_id".to_owned(), app_id.into()),
            ("charset".to_owned(), "utf-8".to_owned()),
            ("sign_type".to_owned(), "RSA2".to_owned()),
            ("format".to_owned(), "json".to_owned()),
            ("version".to_owned(), "1.0".to_owned()),
        ]);

        if let Some(cert_sn) = app_cert_sn {
            let app_cert_sn = app_cert_client::get_cert_sn(cert_sn).unwrap_or(String::from(""));
            params.insert("app_cert_sn".to_owned(), app_cert_sn);
        }
        if let Some(root_cert_sn) = alipay_root_cert_sn {
            let alipay_root_cert_sn =
                app_cert_client::get_root_cert_sn(root_cert_sn).unwrap_or(String::from(""));
            params.insert("alipay_root_cert_sn".to_owned(), alipay_root_cert_sn);
        }
        Self {
            api_url: String::from("https://openapi.alipay.com/gateway.do"),
            request_params: params,
            private_key: private_key.into(),
        }
    }
    fn create_sign(self) -> AlipayResult<HashMap<String, String>> {
        let mut params = self.request_params.clone();
        let mut p = params.iter().collect::<Vec<_>>();
        p.sort_by(|a, b| a.0.cmp(b.0));
        let mut temp: String = String::from("");
        for (key, val) in p.iter() {
            temp.push_str(key);
            temp.push_str("=");
            temp.push_str(val);
            temp.push_str("&");
        }
        temp.pop();

        let private_key = self.get_private_key()?;
        let mut signer = Signer::new(MessageDigest::sha256(), private_key.as_ref())?;
        signer.update(temp.as_bytes())?;
        let sign = base64::encode_block(signer.sign_to_vec()?.as_ref());
        params.insert("sign".to_owned(), sign);
        Ok(params)
    }
    // 设置请求参数，如果参数存在，更新参数，不存在则插入参数
    fn set_request_params<S: Into<String>>(&mut self, key: S, val: String) {
        let k = key.into();
        if let Some(value) = self.request_params.get_mut(&k.clone()) {
            *value = val;
        } else {
            self.request_params.insert(k, val);
        }
    }
    /// 设置公共参数
    ///
    /// 值为None会被过滤掉
    ///
    /// 如果参数已存在，那么会覆盖原先的参数值
    ///
    /// Example:
    /// ```rust
    /// #[derive(AlipayParam)]
    /// struct PublicParams {
    ///     app_id: String,
    ///     method: Option<String>,
    ///     charset: String,
    ///     sign_type: String,
    ///     sign: Option<String>,
    ///     timestamp: Option<String>,
    ///     version: String,
    ///     biz_content: Option<String>,
    /// }
    ///     let public_params = PublicParams {
    ///         app_id: "2021002182623971".to_owned(),
    ///         method: None,
    ///         charset: "utf-8".to_owned(),
    ///         sign_type: "RSA2".to_owned(),
    ///         sign: None,
    ///         timestamp: None,
    ///         version: "1.0".to_owned(),
    ///         biz_content: None,
    ///     };
    ///     client.set_public_params(public_params);
    /// ```
    pub fn set_public_params<T: AlipayParam>(&mut self, args: T) {
        let params = args.to_map();
        for (key, val) in params.iter() {
            match val {
                FieldValue::Null => continue,
                _ => {
                    self.set_request_params(key, val.to_string());
                }
            }
        }
    }
    /// 异步请求
    ///
    /// 支付宝的官方接口都可以使用此函数访问
    ///
    /// Example:
    /// ```rust
    ///    let client = alipay::Client::new(
    ///         "2021002182623971",
    ///         "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
    ///         Some("appCertPublicKey_2021002182623971.crt"),
    ///         Some("alipayRootCert.crt")
    ///     );
    ///     let data:serde_json::Value = client
    ///         .post("alipay.fund.trans.uni.transfer", transfer)
    ///         .await.unwrap();
    /// ```
    pub async fn post<S: Into<String>, T: Serialize, R: DeserializeOwned>(
        self,
        method: S,
        biz_content: T,
    ) -> AlipayResult<R> {
        self.sync_post(method, biz_content)
    }
    /// 没有参数的异步请求
    pub async fn no_param_post<S: Into<String>, R: DeserializeOwned>(
        self,
        method: S,
    ) -> AlipayResult<R> {
        self.alipay_post(method, None)
    }
    /// 同步请求
    pub fn sync_post<S: Into<String>, T: Serialize, R: DeserializeOwned>(
        self,
        method: S,
        biz_content: T,
    ) -> AlipayResult<R> {
        self.alipay_post(method, Some(serde_json::to_string(&biz_content)?))
    }
    fn alipay_post<S: Into<String>, R: DeserializeOwned>(
        mut self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<R> {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.set_request_params("timestamp", now);
        self.set_request_params("method", method.into());
        if let Some(biz_content) = biz_content {
            self.set_request_params("biz_content", biz_content);
        }
        let url = self.clone().api_url;
        let params: Vec<(String, String)> = self.create_sign()?.into_iter().collect();
        let params = serde_urlencoded::to_string(params)?;
        let res = ureq::post(&url)
            .set(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .send_string(&params)?;

        Ok(res.into_json::<R>()?)
    }
    fn get_private_key(self) -> AlipayResult<PKey<Private>> {
        let cert_content = base64::decode_block(self.private_key.as_str())?;
        let rsa = Rsa::private_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}
