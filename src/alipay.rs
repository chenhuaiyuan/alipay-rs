use crate::app_cert_client;
use crate::error::AlipayResult;
use crate::param::{AlipayParam, FieldValue};
use crate::request_param::RequestParam;
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
    /// app_id: 可在支付宝控制台 -> 我的应用 中查看  
    /// private_key: 支付宝开放平台开发助手生成的应用私钥  
    /// app_cert_sn: 在应用的 开发设置 -> 开发信息 -> 接口加签方式 中获取  
    /// alipay_root_cert_sn: 同上  
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
            let app_cert_sn = app_cert_client::get_cert_sn_from_content(cert_sn.to_owned())
                .unwrap_or(String::from(""));
            params.insert("app_cert_sn".to_owned(), app_cert_sn);
        }
        if let Some(root_cert_sn) = alipay_root_cert_sn {
            let alipay_root_cert_sn =
                app_cert_client::get_root_cert_sn_from_content(root_cert_sn.to_owned())
                    .unwrap_or(String::from(""));
            params.insert("alipay_root_cert_sn".to_owned(), alipay_root_cert_sn);
        }
        Self {
            api_url: "https://openapi.alipay.com/gateway.do".to_owned(),
            request_params: RequestParam::new(params),
            private_key: private_key.into(),
            other_params: RequestParam::default(),
        }
    }
    /// app_id: 可在支付宝控制台 -> 我的应用 中查看  
    /// private_key_path: 支付宝开放平台开发助手生成的应用私钥文件  
    /// app_cert_sn: 在应用的 开发设置 -> 开发信息 -> 接口加签方式 中获取  
    /// alipay_root_cert_sn: 同上  
    pub fn neo<S: Into<String>>(
        app_id: S,
        private_key_path: &str,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Client {
        let private_key =
            app_cert_client::get_private_key_from_file(private_key_path).unwrap_or("".to_string());
        Client::new(app_id.into(), private_key, app_cert_sn, alipay_root_cert_sn)
    }
    fn create_params(&mut self) -> AlipayResult<HashMap<String, String>> {
        let mut params = self.request_params.clone().inner();

        let other_params = self.other_params.clone().inner();
        for (key, val) in other_params {
            params.insert(key, val);
        }
        self.other_params.clear();

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
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(temp.as_bytes())?;
        let sign = base64::encode_block(signer.sign_to_vec()?.as_ref());
        params.insert("sign".to_owned(), sign);
        Ok(params)
    }
    // 设置请求参数，如果参数存在，更新参数，不存在则插入参数
    fn set_request_params<S: Into<String>>(&mut self, key: S, val: String) {
        // let k = key.into();
        self.request_params.save(key.into(), val);
        // let params = self.request_params.0.get_mut();
        // if let Some(value) = params.get_mut(&k.clone()) {
        //     *value = val;
        // } else {
        //     params.insert(k, val);
        //     // self.request_params = Arc::new(params.as_ref());
        // }
    }
    /// 设置公共参数
    ///
    /// 值为None或者参数不存在会被过滤掉  
    /// 可设置的参数有 app_id，charset，sign_type，format，version，method，timestamp，sign
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
    /// }
    ///     let public_params = PublicParams {
    ///         app_id: "20210xxxxxxxxxxx".to_owned(),
    ///         method: None,
    ///         charset: "utf-8".to_owned(),
    ///         sign_type: "RSA2".to_owned(),
    ///         sign: None,
    ///         timestamp: None,
    ///         version: "1.0".to_owned(),
    ///     };
    ///     client.set_public_params(public_params);
    /// ```
    pub fn set_public_params<T: AlipayParam>(&mut self, args: T) {
        let params = args.to_map();

        for (key, val) in params {
            match val {
                FieldValue::Null => continue,
                _ => {
                    self.request_params.set(key, val.to_string());
                }
            }
        }
    }
    /// 添加公共参数  
    /// ```rust
    /// #[derive(AlipayParam)]
    /// struct ImageUpload {
    ///     image_type: String,
    ///     image_name: String,
    /// }
    ///
    /// ...
    ///
    /// let image = ImageUpload {
    ///     image_type: "png".to_owned(),
    ///     image_name: "test".to_owned(),
    /// };
    ///
    /// ...
    ///
    /// client.add_public_params(image);
    /// ```
    pub fn add_public_params<T: AlipayParam>(&mut self, args: T) {
        let params = args.to_map();

        for (key, val) in params {
            match val {
                FieldValue::Null => continue,
                _ => {
                    self.other_params.add(key, val.to_string());
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
    ///    let client = alipay_rs::Client::new(
    ///         "20210xxxxxxxxxxx",
    ///         include_str!("../私钥.txt"),
    ///         Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    ///         Some(include_str!("../alipayRootCert.crt"))
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
    /// 文件上传  
    /// method: 接口名称  
    /// key: 文件参数名  
    /// file_name: 文件名  
    /// file_content: 文件内容  
    ///
    /// ```rust
    /// #[derive(AlipayParam)]
    /// struct Image {
    ///     image_type: String,
    ///     image_name: String,
    /// }
    /// let file = std::fs::read("./test.png").unwrap();
    /// let image = Image {
    ///     image_type: "png".to_owned(),
    ///     image_name: "test".to_owned(),
    /// };
    /// let mut client = ...;
    /// client.add_public_params(image);
    /// let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
    /// println!("{:?}", data);
    /// ```
    pub async fn post_file<'a, S: Into<String>, D: DeserializeOwned>(
        self,
        method: S,
        key: &'a str,
        file_name: &'a str,
        file_content: &[u8],
    ) -> AlipayResult<D> {
        let mut multi = multipart::client::lazy::Multipart::new();
        multi.add_stream(key, file_content, Some(file_name), None);
        let mdata = multi.prepare()?;
        let mut url = self.api_url.clone();
        let params = self.build_params(method, None)?;
        url.push('?');
        url.push_str(params.as_str());
        let res = ureq::post(url.as_str())
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", mdata.boundary()),
            )
            .send(mdata)?;
        Ok(res.into_json::<D>()?)
    }

    fn build_params<S: Into<String>>(
        mut self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<String> {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.set_request_params("timestamp", now);
        self.set_request_params("method", method.into());
        if let Some(biz_content) = biz_content {
            self.other_params.add("biz_content".to_owned(), biz_content);
        }
        let params = self.create_params()?;
        let params = serde_urlencoded::to_string(params)?;
        Ok(params)
    }
    fn alipay_post<S: Into<String>, R: DeserializeOwned>(
        self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<R> {
        let url = self.api_url.clone();
        let params = self.build_params(method, biz_content)?;
        let res = ureq::post(url.as_str())
            .set(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .send_string(&params)?;

        Ok(res.into_json::<R>()?)
    }
    fn get_private_key(&self) -> AlipayResult<PKey<Private>> {
        let cert_content = base64::decode_block(self.private_key.as_str())?;
        let rsa = Rsa::private_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}
