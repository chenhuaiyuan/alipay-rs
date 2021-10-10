use std::collections::HashMap;

use crate::app_cert_client;
use crate::error::AlipayResult;
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::Signer,
};
use serde::Serialize;
use serde_json::{json, value::Value};

#[derive(Debug, Serialize, Clone)]
pub struct RequestParams {
    app_id: String,
    method: Option<String>,
    charset: String,
    sign_type: String,
    format: String,
    // sign: Option<String>,
    timestamp: Option<String>,
    version: String,
    alipay_root_cert_sn: String,
    app_cert_sn: String,
    biz_content: Option<Value>,
}
#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
    request_params: RequestParams,
    private_key: String,
}
impl Client {
    pub fn new<S: Into<String>>(
        app_id: S,
        app_cert_sn: &str,
        alipay_root_cert_sn: &str,
        private_key: S,
    ) -> Client {
        let app_cert_sn = app_cert_client::get_cert_sn(app_cert_sn).unwrap_or(String::from(""));
        let alipay_root_cert_sn =
            app_cert_client::get_root_cert_sn(alipay_root_cert_sn).unwrap_or(String::from(""));
        let params = RequestParams {
            app_id: app_id.into(),
            method: None,
            charset: String::from("UTF-8"),
            sign_type: String::from("RSA2"),
            format: String::from("JSON"),
            // sign: None,
            timestamp: None,
            version: String::from("1.0"),
            app_cert_sn,
            alipay_root_cert_sn,
            biz_content: None,
        };
        Self {
            api_url: String::from("https://openapi.alipay.com/gateway.do"),
            request_params: params,
            private_key: private_key.into(),
        }
    }
    fn generate_form_params(&mut self) -> AlipayResult<HashMap<&str, String>> {
        let mut params = HashMap::new();
        let request_params = self.request_params.clone();
        params.insert("app_id", request_params.app_id);
        params.insert("method", request_params.method.unwrap_or(String::from("")));
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
        params.insert("app_cert_sn", request_params.app_cert_sn);
        params.insert("alipay_root_cert_sn", request_params.alipay_root_cert_sn);
        let biz_content = request_params
            .biz_content
            .unwrap_or(Value::Null)
            .to_string();
        params.insert("biz_content", biz_content);
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
        println!("{}", temp);
        let private_key = self.clone().get_private_key()?;
        let mut signer = Signer::new(MessageDigest::sha256(), private_key.as_ref())?;
        signer.update(temp.as_bytes())?;
        let sign = base64::encode(signer.sign_to_vec()?);
        // println!("{}", sign);
        // self.request_params.sign = Some(sign.clone());
        params.insert("sign", sign);
        Ok(params)
    }
    pub async fn post<S: Into<String>, T: Serialize>(
        &mut self,
        method: S,
        biz_content: T,
    ) -> AlipayResult<()> {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.request_params.timestamp = Some(now);
        self.request_params.method = Some(method.into());
        self.request_params.biz_content = Some(json!(biz_content));
        let url = self.clone().api_url;
        let params = self.generate_form_params()?;
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .form(&params)
            .send()
            .await?;

        println!("{:?}", res.text().await?);
        Ok(())
    }
    fn get_private_key(self) -> AlipayResult<PKey<Private>> {
        let cert_content = base64::decode(self.private_key)?;
        let rsa = Rsa::private_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}
