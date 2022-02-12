use crate::{app_cert_client, error::AlipayResult, AlipayParam, Client, FieldValue};
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::Signer,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap};

fn get_hour_min_sec(timestamp: u64) -> (i32, i32, i32) {
    let hour = (timestamp % (24 * 3600)) / 3600 + 8;
    let min = (timestamp % 3600) / 60;
    let sec = (timestamp % 3600) % 60;
    (hour as i32, min as i32, sec as i32)
}

fn get_moth_day(is_leap_year: bool, mut days: i32) -> (i32, i32) {
    let p_moth: Vec<i32> = if is_leap_year {
        vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut day = 0;
    let mut moth = 0;

    for i in 0..12 {
        let temp = days - p_moth[i];
        if temp <= 0 {
            moth = i + 1;
            day = if temp == 0 { p_moth[i] } else { days };
            break;
        }
        days = temp;
    }
    (moth as i32, day)
}

fn datetime() -> AlipayResult<String> {
    use std::time::SystemTime;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let days = 24 * 3600;
    let four_years = 365 * 3 + 366;
    let days = timestamp / days + (if (timestamp % days) != 0 { 1 } else { 0 });
    let year_4 = days / four_years;
    let mut remain = days % four_years;
    let mut year = 1970 + year_4 * 4;

    let mut is_leap_year = false;

    if 365 <= remain && remain < 365 * 2 {
        year += 1;
        remain -= 365;
    } else if 365 * 2 <= remain && remain < 365 * 3 {
        year += 2;
        remain -= 365 * 2;
    } else if 365 * 3 <= remain {
        year += 3;
        remain -= 365 * 3;
        is_leap_year = true;
    }

    let (moth, day) = get_moth_day(is_leap_year, remain as i32);
    let (h, m, s) = get_hour_min_sec(timestamp);
    Ok(format!(
        "{}-{:>02}-{:>02} {:>02}:{:>02}:{:>02}",
        year, moth, day, h, m, s,
    ))
}

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
            request_params: RefCell::new(params),
            private_key: private_key.into(),
            other_params: RefCell::new(HashMap::new()),
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
        let mut cert_sn: String = String::from("");
        if let Some(cert_sn_path) = app_cert_sn {
            cert_sn = app_cert_client::get_cert_sn(cert_sn_path).unwrap_or(String::from(""));
        }
        let mut root_cert_sn: String = String::from("");
        if let Some(root_cert_sn_path) = alipay_root_cert_sn {
            root_cert_sn =
                app_cert_client::get_root_cert_sn(root_cert_sn_path).unwrap_or(String::from(""));
        }
        Client::new(
            app_id.into(),
            private_key,
            Some(&cert_sn),
            Some(&root_cert_sn),
        )
    }
    fn create_params(&mut self) -> AlipayResult<String> {
        let mut request_params = self.request_params.borrow_mut();
        let request_params_len = request_params.len();

        let mut other_params = self.other_params.borrow_mut();
        let other_params_len = other_params.len();
        let mut params: Vec<(String, String)> =
            Vec::with_capacity(request_params_len + other_params_len);

        for (key, val) in request_params.iter() {
            params.push((key.to_string(), val.to_string()));
        }
        request_params.clear();

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

        let private_key = self.get_private_key()?;
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(temp.as_bytes())?;
        let sign = base64::encode_block(signer.sign_to_vec()?.as_ref());
        params.push(("sign".to_owned(), sign));
        Ok(serde_urlencoded::to_string(params)?)
    }
    // 设置请求参数，如果参数存在，更新参数，不存在则插入参数
    fn set_request_params<S: Into<String>>(&mut self, key: S, val: String) {
        let key = key.into();
        let mut request_params = self.request_params.borrow_mut();
        if let Some(value) = request_params.get_mut(&key) {
            *value = val;
        } else {
            request_params.insert(key, val);
        }
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
                    if let Some(value) = self.request_params.borrow_mut().get_mut(&key) {
                        *value = val.to_string();
                    }
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
                    self.other_params.borrow_mut().insert(key, val.to_string());
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
        mut self,
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
        Ok(self.create_params()?)
    }
    fn alipay_post<S: Into<String>, R: DeserializeOwned>(
        self,
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
}
