use crate::{
    error::AlipayResult, response::Response, util::datetime, AlipayParams, BoxFuture, MutCli, Sign,
};
use futures::FutureExt;
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::{Signer, Verifier},
};
use serde_json::Value;
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub struct ClientWithParams {
    public_key: String,
    private_key: String,
    request_params: HashMap<String, String>,
    other_params: HashMap<String, Value>,
    sandbox: bool,
}

impl ClientWithParams {
    pub(crate) fn new(
        public_key: String,
        private_key: String,
        request_params: HashMap<String, String>,
        other_params: HashMap<String, Value>,
        sandbox: bool,
    ) -> Self {
        Self {
            public_key,
            private_key,
            request_params,
            other_params,
            sandbox,
        }
    }
    /// 设置/添加公共参数
    ///
    ///
    /// Example:
    /// ```rust
    /// #[derive(AlipayParams)]
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
    pub fn set_public_params<T>(&mut self, args: T) -> &mut Self
    where
        T: AlipayParams,
    {
        let params = args.to_alipay_value().to_json_value();
        if let Value::Object(v) = params {
            for (key, val) in v {
                self.other_params.insert(key, val);
            }
        }
        self
    }

    fn alipay_post<S: Into<String>>(
        &mut self,
        method: S,
        biz_content: Option<String>,
    ) -> AlipayResult<Response> {
        let url = if !self.sandbox {
            "https://openapi.alipay.com/gateway.do"
        } else {
            "https://openapi.alipaydev.com/gateway.do"
        };
        let params = self.build_params(method, biz_content)?;
        let res = ureq::post(url)
            .set(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .send_string(&params)?;

        Ok(Response::new(res))
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
                .insert("biz_content".to_owned(), Value::from(biz_content));
        }
        self.create_params()
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

impl MutCli for ClientWithParams {
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
    ///
    ///     // 现在除了AlipayParams宏来设置参数外，还可以通过vec, hashmap, array, tuple来存储参数
    ///     // 如果没有参数可以传“()”，比如：client.post("method", ())
    ///     let params = ("app_id", "20210xxxxxxxxxxx");
    ///     let params = [("image_type", "png"), ("image_name", "test")];
    ///     let params = vec![("image_type", "png"), ("image_name", "test")];
    ///     let params = HashMap::from([("image_type", "png"), ("image_name", "test")]);
    /// ```
    fn post<'a, S, T>(
        &'a mut self,
        method: S,
        biz_content: T,
    ) -> BoxFuture<'a, AlipayResult<Response>>
    where
        S: Into<String> + Send + 'a,
        T: AlipayParams + Send + 'a,
    {
        async move { self.sync_post(method, biz_content) }.boxed()
    }
    /// 没有参数的异步请求
    /// 此函数后期考虑放弃，请调用post函数。
    /// 如果没有参数，可以这样调用post，post("method", ()) 或 post("method", None)
    fn no_param_post<'a, S>(&'a mut self, method: S) -> BoxFuture<'a, AlipayResult<Response>>
    where
        S: Into<String> + Send + 'a,
    {
        async move { self.alipay_post(method, None) }.boxed()
    }
    /// 同步请求
    fn sync_post<'a, S, T>(&'a mut self, method: S, biz_content: T) -> AlipayResult<Response>
    where
        S: Into<String> + Send + 'a,
        T: AlipayParams + Send + 'a,
    {
        let biz_content = biz_content.to_alipay_value();
        if biz_content.is_null() {
            self.alipay_post::<S>(method, None)
        } else {
            self.alipay_post::<S>(
                method,
                Some(serde_json::to_string(&biz_content.to_json_value())?),
            )
        }
    }

    /// 文件上传
    /// method: 接口名称
    /// key: 文件参数名
    /// file_name: 文件名
    /// file_content: 文件内容
    ///
    /// ```rust
    /// #[derive(AlipayParams)]
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
        &'a mut self,
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
            let mut url = if !self.sandbox {
                "https://openapi.alipay.com/gateway.do".to_owned()
            } else {
                "https://openapi.alipaydev.com/gateway.do".to_owned()
            };
            let params = self.build_params(method, None)?;
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

impl Sign for ClientWithParams {
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
