use std::collections::HashMap;

use crate::app_cert_client;
use crate::error::AlipayResult;
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::Signer,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
    request_params: HashMap<String, String>,
    private_key: String,
}
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
    pub fn set_request_params<S: Into<String>>(&mut self, key: S, val: String) {
        let k = key.into();
        if let Some(value) = self.request_params.get_mut(&k.clone()) {
            *value = val;
        } else {
            self.request_params.insert(k, val);
        }
    }
    // 异步请求
    pub async fn post<S: Into<String>, T: Serialize, R: DeserializeOwned>(
        self,
        method: S,
        biz_content: T,
    ) -> AlipayResult<R> {
        self.sync_post(method, biz_content)
    }
    // 没有参数的异步请求
    pub async fn no_params_post<S: Into<String>, R: DeserializeOwned>(
        self,
        method: S,
    ) -> AlipayResult<R> {
        self.alipay_post(method, None)
    }
    // 同步请求
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
