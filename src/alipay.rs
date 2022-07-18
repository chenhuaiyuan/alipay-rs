use crate::{app_cert_client, error::AlipayResult, util::datetime, PublicParams};
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::{Signer, Verifier},
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub trait Sign {
    fn sign(&self, params: &str) -> AlipayResult<String>;
    fn verify(&self, source: &str, signature: &str) -> AlipayResult<bool>;
}

#[derive(Debug)]
pub struct Client {
    public_key: String,
    private_key: String,
    request_params: HashMap<String, String>,
    other_params: HashMap<String, String>,
}

impl Client {
    /// app_id: 可在支付宝控制台 -> 我的应用 中查看
    /// public_key: 支付宝开放平台开发助手生成的应用公钥钥文件
    /// private_key: 支付宝开放平台开发助手生成的应用私钥
    /// app_cert_sn: 在应用的 开发设置 -> 开发信息 -> 接口加签方式 中获取
    /// alipay_root_cert_sn: 同上
    pub fn new<S: Into<String>>(
        app_id: S,
        public_key: S,
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
            let app_cert_sn = app_cert_client::get_cert_sn_from_content(cert_sn.as_ref())
                .unwrap_or_else(|_| String::from(""));
            params.insert("app_cert_sn".to_owned(), app_cert_sn);
        }
        if let Some(root_cert_sn) = alipay_root_cert_sn {
            let alipay_root_cert_sn = app_cert_client::get_root_cert_sn_from_content(root_cert_sn)
                .unwrap_or_else(|_| String::from(""));
            params.insert("alipay_root_cert_sn".to_owned(), alipay_root_cert_sn);
        }
        Self {
            public_key: public_key.into(),
            private_key: private_key.into(),
            request_params: params,
            other_params: HashMap::new(),
        }
    }

    /// app_id: 可在支付宝控制台 -> 我的应用 中查看
    /// public_key_path: 支付宝开放平台开发助手生成的应用公钥钥文件
    /// private_key_path: 支付宝开放平台开发助手生成的应用私钥文件
    /// app_cert_sn: 在应用的 开发设置 -> 开发信息 -> 接口加签方式 中获取
    /// alipay_root_cert_sn: 同上
    pub fn neo<S: Into<String>>(
        app_id: S,
        public_key_path: &str,
        private_key_path: &str,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Client {
        let public_key =
            app_cert_client::get_file_content(public_key_path).unwrap_or_else(|_| String::from(""));
        let private_key = app_cert_client::get_file_content(private_key_path)
            .unwrap_or_else(|_| String::from(""));
        let mut cert_sn: String = String::from("");
        if let Some(cert_sn_path) = app_cert_sn {
            cert_sn =
                app_cert_client::get_cert_sn(cert_sn_path).unwrap_or_else(|_| String::from(""));
        }
        let mut root_cert_sn: String = String::from("");
        if let Some(root_cert_sn_path) = alipay_root_cert_sn {
            root_cert_sn = app_cert_client::get_root_cert_sn(root_cert_sn_path)
                .unwrap_or_else(|_| String::from(""));
        }
        Client::new(
            app_id.into(),
            public_key,
            private_key,
            Some(&cert_sn),
            Some(&root_cert_sn),
        )
    }

    // 通过config创建client
    pub(crate) fn new_from_config(
        request_params: HashMap<String, String>,
        public_key: String,
        private_key: String,
    ) -> Client {
        Client {
            public_key,
            private_key,
            request_params,
            other_params: HashMap::new(),
        }
    }

    fn create_params(&mut self) -> AlipayResult<String> {
        let request_params_len = self.request_params.len();

        let other_params = self.other_params.borrow_mut();
        let other_params_len = other_params.len();
        let mut params: Vec<(String, String)> =
            Vec::with_capacity(request_params_len + other_params_len);

        for (key, val) in self.request_params.iter() {
            if other_params.get(key).is_none() {
                params.push((key.to_string(), val.to_string()));
            }
        }

        for (key, val) in other_params.iter() {
            params.push((key.to_string(), val.to_string()));
        }
        other_params.clear();

        params.sort_by(|a, b| a.0.cmp(&b.0));
        let mut temp = String::new();
        for (key, val) in params.iter() {
            temp.push_str(key);
            temp.push('=');
            temp.push_str(val);
            temp.push('&');
        }
        temp.pop();

        let sign = self.sign(&temp)?;
        params.push(("sign".to_owned(), sign));
        Ok(serde_urlencoded::to_string(params)?)
    }

    // 设置请求参数，如果参数存在，更新参数，不存在则插入参数
    fn set_request_params<S: Into<String>>(&mut self, key: S, val: String) {
        let key = key.into();
        let request_params = self.request_params.borrow_mut();
        if let Some(value) = request_params.get_mut(&key) {
            *value = val;
        } else {
            request_params.insert(key, val);
        }
    }

    /// 设置/添加公共参数
    ///
    /// set_public_params 和 add_public_params以合并，现在统一使用set_public_params
    ///
    ///
    /// Example:
    /// ```rust
    /// #[derive(AlipayParam)]
    /// struct PublicParams {
    ///     app_id: String,
    ///     charset: String,
    ///     sign_type: String,
    ///     version: String,
    /// }
    ///
    /// ......
    ///
    ///     let public_params = PublicParams {
    ///         app_id: "20210xxxxxxxxxxx".to_owned(),
    ///         charset: "utf-8".to_owned(),
    ///         sign_type: "RSA2".to_owned(),
    ///         version: "1.0".to_owned(),
    ///     };
    ///
    ///     // 也可以通过vec, hashmap, array, tuple来设置公共参数
    ///     let public_params = ("app_id", "20210xxxxxxxxxxx");
    ///     let public_params = [("image_type", "png"), ("image_name", "test")];
    ///     let public_params = vec![("image_type", "png"), ("image_name", "test")];
    ///     let public_params = HashMap::from([("image_type", "png"), ("image_name", "test")]);
    ///
    ///     client.set_public_params(public_params);
    /// ```
    pub fn set_public_params<T: PublicParams>(&mut self, args: T) {
        let params = args.to_hash_map();

        for (key, val) in params {
            if let Some(value) = self.other_params.get_mut(&key) {
                *value = val;
            } else {
                self.other_params.insert(key, val);
            }
        }
    }

    /// 添加公共参数
    /// 此函数打算弃用，请改用set_public_params
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
    #[deprecated(
        since = "0.3.1",
        note = "Please use the set_public_params function instead"
    )]
    pub fn add_public_params<T: PublicParams>(&mut self, args: T) {
        let params = args.to_hash_map();

        for (key, val) in params {
            self.other_params.insert(key, val);
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
    ///         include_str!("../公钥.txt"),
    ///         include_str!("../私钥.txt"),
    ///         Some(include_str!("../appCertPublicKey_20210xxxxxxxxxxx.crt")),
    ///         Some(include_str!("../alipayRootCert.crt"))
    ///     );
    ///     let data:serde_json::Value = client
    ///         .post("alipay.fund.trans.uni.transfer", transfer)
    ///         .await.unwrap();
    /// ```
    pub async fn post<S: Into<String>, T: Serialize, R: DeserializeOwned>(
        &mut self,
        method: S,
        biz_content: T,
    ) -> AlipayResult<R> {
        self.sync_post(method, biz_content)
    }
    /// 没有参数的异步请求
    pub async fn no_param_post<S: Into<String>, R: DeserializeOwned>(
        &mut self,
        method: S,
    ) -> AlipayResult<R> {
        self.alipay_post(method, None)
    }
    /// 同步请求
    pub fn sync_post<S: Into<String>, T: Serialize, R: DeserializeOwned>(
        &mut self,
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
    /// client.set_public_params(image);
    /// let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
    /// println!("{:?}", data);
    /// ```
    pub async fn post_file<'a, S: Into<String>, D: DeserializeOwned>(
        &mut self,
        method: S,
        key: &'a str,
        file_name: &'a str,
        file_content: &[u8],
    ) -> AlipayResult<D> {
        let mut multi = multipart::client::lazy::Multipart::new();
        multi.add_stream(key, file_content, Some(file_name), None);
        let mdata = multi.prepare()?;
        let mut url = "https://openapi.alipay.com/gateway.do".to_owned();
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
        &mut self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<String> {
        let now = datetime()?;
        self.set_request_params("timestamp", now);
        self.set_request_params("method", method.into());
        if let Some(biz_content) = biz_content {
            self.other_params
                .borrow_mut()
                .insert("biz_content".to_owned(), biz_content);
        }
        self.create_params()
    }

    fn alipay_post<S: Into<String>, R: DeserializeOwned>(
        &mut self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<R> {
        let url = "https://openapi.alipay.com/gateway.do";
        let params = self.build_params(method, biz_content)?;
        let res = ureq::post(url)
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
    fn get_public_key(&self) -> AlipayResult<PKey<Public>> {
        let cert_content = base64::decode_block(self.public_key.as_str())?;
        let rsa = Rsa::public_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}

impl Sign for Client {
    fn sign(&self, params: &str) -> AlipayResult<String> {
        let private_key = self.get_private_key()?;
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(params.as_bytes())?;
        let sign = base64::encode_block(signer.sign_to_vec()?.as_ref());
        Ok(sign)
    }
    fn verify(&self, source: &str, signature: &str) -> AlipayResult<bool> {
        let public_key = self.get_public_key()?;
        let sign = base64::decode_block(signature)?;
        let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
        verifier.update(source.as_bytes())?;
        Ok(verifier.verify(sign.as_slice())?)
    }
}
