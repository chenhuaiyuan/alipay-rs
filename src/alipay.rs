use std::collections::HashMap;

use crate::app_cert_client;
use crate::error::AlipayResult;
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::Signer,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct RequestParams {
    app_id: String,
    method: Option<String>,
    charset: String,
    sign_type: String,
    format: String,
    timestamp: Option<String>,
    version: String,
    alipay_root_cert_sn: Option<String>,
    app_cert_sn: Option<String>,
    biz_content: Option<String>,
    auth_token: Option<String>,
    notify_url: Option<String>,
    app_auth_token: Option<String>,
}
impl RequestParams {
    pub fn set_alipay_root_cent_sn(&mut self, root_cent_sn: &str) {
        let alipay_root_cert_sn =
            app_cert_client::get_root_cert_sn(root_cent_sn).unwrap_or(String::from(""));
        self.alipay_root_cert_sn = Some(alipay_root_cert_sn);
    }
    pub fn set_app_cert_sn(&mut self, cert_sn: &str) {
        let app_cert_sn = app_cert_client::get_cert_sn(cert_sn).unwrap_or(String::from(""));
        self.app_cert_sn = Some(app_cert_sn);
    }
    pub fn set_biz_content<T: Serialize>(&mut self, biz_content: T) -> AlipayResult<()> {
        self.biz_content = Some(serde_json::to_string(&biz_content)?);
        Ok(())
    }
    pub fn set_notify_url<S: Into<String>>(&mut self, url: S) {
        self.notify_url = Some(url.into());
    }
}
#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
    pub request_params: RequestParams,
    private_key: String,
}
impl Client {
    pub fn new<S: Into<String>>(
        app_id: S,
        private_key: S,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Client {
        let mut params = RequestParams {
            app_id: app_id.into(),
            method: None,
            charset: String::from("utf-8"),
            sign_type: String::from("RSA2"),
            format: String::from("json"),
            timestamp: None,
            version: String::from("1.0"),
            app_cert_sn: None,
            alipay_root_cert_sn: None,
            biz_content: None,
            auth_token: None,
            notify_url: None,
            app_auth_token: None,
        };
        if let Some(cert_sn) = app_cert_sn {
            let app_cert_sn = app_cert_client::get_cert_sn(cert_sn).unwrap_or(String::from(""));
            params.app_cert_sn = Some(app_cert_sn);
        }
        if let Some(root_cert_sn) = alipay_root_cert_sn {
            let alipay_root_cert_sn =
                app_cert_client::get_root_cert_sn(root_cert_sn).unwrap_or(String::from(""));
            params.alipay_root_cert_sn = Some(alipay_root_cert_sn);
        }
        Self {
            api_url: String::from("https://openapi.alipay.com/gateway.do"),
            request_params: params,
            private_key: private_key.into(),
        }
    }
    fn generate_to_params(&mut self) -> AlipayResult<HashMap<&str, String>> {
        let mut params = HashMap::new();
        let request_params = self.request_params.clone();
        params.insert("app_id", request_params.app_id);
        if let Some(method) = request_params.method {
            params.insert("method", method);
        }
        params.insert("charset", request_params.charset);
        params.insert("format", request_params.format);
        params.insert("sign_type", request_params.sign_type);
        params.insert(
            "timestamp",
            request_params
                .timestamp
                .unwrap_or(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        );
        params.insert("version", request_params.version);
        if let Some(app_cert_sn) = request_params.app_cert_sn {
            params.insert("app_cert_sn", app_cert_sn);
        }
        if let Some(alipay_root_cert_sn) = request_params.alipay_root_cert_sn {
            params.insert("alipay_root_cert_sn", alipay_root_cert_sn);
        }
        if let Some(biz_content) = request_params.biz_content {
            params.insert("biz_content", biz_content);
        }
        if let Some(auth_token) = request_params.auth_token {
            params.insert("auth_token", auth_token);
        }
        if let Some(notify_url) = request_params.notify_url {
            params.insert("notify_url", notify_url);
        }
        if let Some(app_auth_token) = request_params.app_auth_token {
            params.insert("app_auth_token", app_auth_token);
        }
        let mut keys = params.keys().copied().collect::<Vec<_>>();
        keys.sort_unstable();
        let mut temp: String = String::from("");
        for key in keys.iter() {
            temp.push_str(key);
            temp.push_str("=");
            temp.push_str(&params[key]);
            temp.push_str("&");
        }
        temp.pop();

        let private_key = self.clone().get_private_key()?;
        let mut signer = Signer::new(MessageDigest::sha256(), private_key.as_ref())?;
        signer.update(temp.as_bytes())?;
        let sign = base64::encode(signer.sign_to_vec()?);
        // println!("{}", sign);
        // self.request_params.sign = Some(sign.clone());
        params.insert("sign", sign);
        Ok(params)
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
        self.request_params.timestamp = Some(now);
        self.request_params.method = Some(method.into());
        self.request_params.biz_content = biz_content;
        let url = self.clone().api_url;
        let params: Vec<(&str, String)> = self.generate_to_params()?.into_iter().collect();
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
        let cert_content = base64::decode(self.private_key)?;
        let rsa = Rsa::private_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}
