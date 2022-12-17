use crate::{
    app_cert_client, client_builder::ClientBuilder, error::AlipayResult, response::Response,
    util::datetime, BoxFuture, Cli, ClientWithParams, PublicParams, Sign,
};
use futures::FutureExt;
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::{Signer, Verifier},
};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Client {
    public_key: String,
    private_key: String,
    request_params: HashMap<String, String>,
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
        app_cert_sn: Option<S>,
        alipay_root_cert_sn: Option<S>,
    ) -> Client {
        let mut params: HashMap<String, String> = HashMap::from([
            ("app_id".to_owned(), app_id.into()),
            ("charset".to_owned(), "utf-8".to_owned()),
            ("sign_type".to_owned(), "RSA2".to_owned()),
            ("format".to_owned(), "json".to_owned()),
            ("version".to_owned(), "1.0".to_owned()),
        ]);

        if let Some(cert_sn) = app_cert_sn {
            let app_cert_sn = app_cert_client::get_cert_sn_from_content(cert_sn.into().as_ref())
                .unwrap_or_else(|_| String::from(""));
            params.insert("app_cert_sn".to_owned(), app_cert_sn);
        }
        if let Some(root_cert_sn) = alipay_root_cert_sn {
            let alipay_root_cert_sn =
                app_cert_client::get_root_cert_sn_from_content(&root_cert_sn.into())
                    .unwrap_or_else(|_| String::from(""));
            params.insert("alipay_root_cert_sn".to_owned(), alipay_root_cert_sn);
        }
        Self {
            public_key: public_key.into(),
            private_key: private_key.into(),
            request_params: params,
        }
    }

    /// app_id: 可在支付宝控制台 -> 我的应用 中查看
    /// public_key_path: 支付宝开放平台开发助手生成的应用公钥钥文件
    /// private_key_path: 支付宝开放平台开发助手生成的应用私钥文件
    /// app_cert_sn: 在应用的 开发设置 -> 开发信息 -> 接口加签方式 中获取
    /// alipay_root_cert_sn: 同上
    pub fn neo<S: Into<String>>(
        app_id: S,
        public_key_path: S,
        private_key_path: S,
        app_cert_sn: Option<S>,
        alipay_root_cert_sn: Option<S>,
    ) -> Client {
        let public_key = app_cert_client::get_file_content(&public_key_path.into())
            .unwrap_or_else(|_| String::from(""));
        let private_key = app_cert_client::get_file_content(&private_key_path.into())
            .unwrap_or_else(|_| String::from(""));
        let mut cert_sn: String = String::from("");
        if let Some(cert_sn_path) = app_cert_sn {
            cert_sn = app_cert_client::get_cert_sn(&cert_sn_path.into())
                .unwrap_or_else(|_| String::from(""));
        }
        let mut root_cert_sn: String = String::from("");
        if let Some(root_cert_sn_path) = alipay_root_cert_sn {
            root_cert_sn = app_cert_client::get_root_cert_sn(&root_cert_sn_path.into())
                .unwrap_or_else(|_| String::from(""));
        }
        Client::new(
            app_id.into(),
            public_key,
            private_key,
            Some(cert_sn),
            Some(root_cert_sn),
        )
    }

    /// ```rust
    /// let client = alipay_rs::Client::builder()
    /// .app_id("2021002199679230")
    /// .public_key(include_str!("../公钥.txt"))
    /// .private_key(include_str!("../私钥.txt"))
    /// .app_cert_sn(include_str!("../appCertPublicKey_2021002199679230.crt"))
    /// .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
    /// .finish();
    /// ```
    pub fn builder<'a>() -> ClientBuilder<'a> {
        ClientBuilder::default()
    }

    /// 设置/添加公共参数
    ///
    /// set_public_params 和 add_public_params已合并，现在统一使用set_public_params
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
    pub fn set_public_params<T>(&self, args: T) -> ClientWithParams
    where
        T: PublicParams,
    {
        let params = args.to_hash_map();
        let mut other_params = HashMap::new();

        for (key, val) in params {
            // self.other_params.insert(key, val);
            other_params.insert(key, val);
        }
        ClientWithParams::new(
            self.public_key.clone(),
            self.private_key.clone(),
            self.request_params.clone(),
            other_params,
        )
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
    pub fn add_public_params<T: PublicParams>(&self, args: T) -> ClientWithParams {
        let params = args.to_hash_map();
        let mut other_params = HashMap::new();

        for (key, val) in params {
            // self.other_params.insert(key, val);
            other_params.insert(key, val);
        }
        ClientWithParams::new(
            self.public_key.clone(),
            self.private_key.clone(),
            self.request_params.clone(),
            other_params,
        )
    }

    fn alipay_post<S: Into<String>>(
        &self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<Response> {
        let url = "https://openapi.alipay.com/gateway.do";
        let params = self.build_params(method.into(), biz_content)?;
        let res = ureq::post(url)
            .set(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .send_string(&params)?;

        Ok(Response::new(res))
    }

    fn build_params(&self, method: String, biz_content: Option<String>) -> AlipayResult<String> {
        let request_params_len = self.request_params.len();

        let now = datetime()?;

        let mut params: Vec<(String, String)> = Vec::with_capacity(request_params_len + 3);

        params.push(("timestamp".to_string(), now));
        params.push(("method".to_string(), method));

        for (key, val) in self.request_params.iter() {
            params.push((key.to_string(), val.to_string()));
        }

        if let Some(content) = biz_content {
            params.push(("biz_content".to_string(), content));
        }

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

impl Cli for Client {
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
    ///         .await.unwrap().into_json().unwrap();
    /// ```
    fn post<'a, S, T>(&'a self, method: S, biz_content: T) -> BoxFuture<'a, AlipayResult<Response>>
    where
        S: Into<String> + Send + 'a,
        T: Serialize + Send + 'a,
    {
        async move { self.sync_post::<'a, S, T>(method, biz_content) }.boxed()
    }
    /// 没有参数的异步请求
    fn no_param_post<'a, S>(&'a self, method: S) -> BoxFuture<'a, AlipayResult<Response>>
    where
        S: Into<String> + Send + 'a,
    {
        async move { self.alipay_post::<S>(method, None) }.boxed()
    }
    /// 同步请求
    fn sync_post<'a, S, T>(&'a self, method: S, biz_content: T) -> AlipayResult<Response>
    where
        S: Into<String> + Send + 'a,
        T: Serialize + Send + 'a,
    {
        self.alipay_post::<S>(method, Some(serde_json::to_string(&biz_content)?))
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
    /// let client = ...;
    /// let mut client_with_params = client.set_public_params(image);
    /// let data:serde_json::Value = client_with_params.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap().into_json().unwrap();
    /// println!("{:?}", data);
    /// ```
    fn post_file<'a, S>(
        &'a self,
        method: S,
        key: &'a str,
        file_name: &'a str,
        file_content: &'a [u8],
    ) -> BoxFuture<'a, AlipayResult<Response>>
    where
        S: Into<String> + Send + 'a,
    {
        async move {
            let mut multi = multipart::client::lazy::Multipart::new();
            multi.add_stream(key, file_content, Some(file_name), None);
            let mdata = multi.prepare()?;
            let mut url = "https://openapi.alipay.com/gateway.do".to_owned();
            let params = self.build_params(method.into(), None)?;
            url.push('?');
            url.push_str(params.as_str());
            let res = ureq::post(url.as_str())
                .set(
                    "Content-Type",
                    &format!("multipart/form-data; boundary={}", mdata.boundary()),
                )
                .send(mdata)?;
            Ok(Response::new(res))
        }
        .boxed()
    }
}
