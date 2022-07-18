use crate::{app_cert_client, Client};
use std::{borrow::BorrowMut, collections::HashMap};

#[derive(Default)]
pub struct ConfigBuilder<'a> {
    app_id: Option<&'a str>,
    public_key: Option<&'a str>,
    private_key: Option<&'a str>,
    app_cert_sn: Option<&'a str>,
    alipay_root_cert_sn: Option<&'a str>,
}

impl<'a> ConfigBuilder<'a> {
    /// 添加app_id
    pub fn app_id(&mut self, app_id: &'a str) -> &mut Self {
        self.app_id = Some(app_id);
        self.borrow_mut()
    }
    /// 添加public_key
    pub fn public_key(&mut self, public_key: &'a str) -> &mut Self {
        self.public_key = Some(public_key);
        self.borrow_mut()
    }
    /// 添加private_key
    pub fn private_key(&mut self, private_key: &'a str) -> &mut Self {
        self.private_key = Some(private_key);
        self.borrow_mut()
    }
    // 添加app_cert_sn
    pub fn app_cert_sn(&mut self, app_cert_sn: &'a str) -> &mut Self {
        self.app_cert_sn = Some(app_cert_sn);
        self.borrow_mut()
    }
    /// 添加alipay_root_cert_sn
    pub fn alipay_root_cert_sn(&mut self, alipay_root_cert_sn: &'a str) -> &mut Self {
        self.alipay_root_cert_sn = Some(alipay_root_cert_sn);
        self.borrow_mut()
    }
    pub fn finish(&self) -> Config {
        let app_cert_sn = self.app_cert_sn.map(|cert_sn| {
            app_cert_client::get_cert_sn_from_content(cert_sn.as_ref())
                .unwrap_or_else(|_| String::from(""))
        });
        let alipay_root_cert_sn = self.alipay_root_cert_sn.map(|root_cert_sn| {
            app_cert_client::get_root_cert_sn_from_content(root_cert_sn)
                .unwrap_or_else(|_| String::from(""))
        });
        Config {
            app_id: self.app_id.map_or(String::from(""), String::from),
            public_key: self.public_key.map_or(String::from(""), String::from),
            private_key: self.private_key.map_or(String::from(""), String::from),
            app_cert_sn,
            alipay_root_cert_sn,
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    app_id: String,
    public_key: String,
    private_key: String,
    app_cert_sn: Option<String>,
    alipay_root_cert_sn: Option<String>,
}

impl Config {
    /// 创建config
    /// let config = alipay_rs::Config::builder()
    /// .app_id("2021002199679230")
    /// .public_key(include_str!("../公钥.txt"))
    /// .private_key(include_str!("../私钥.txt"))
    /// .app_cert_sn(include_str!("../appCertPublicKey_2021002199679230.crt"))
    /// .alipay_root_cert_sn(include_str!("../alipayRootCert.crt"))
    /// .finish();
    /// let mut cli = config.get_client();
    pub fn builder<'a>() -> ConfigBuilder<'a> {
        ConfigBuilder::default()
    }
    /// let config = alipay_rs::Config::new(
    ///     "2021002199679230",
    ///     include_str!("../公钥.txt"),
    ///     include_str!("../私钥.txt"),
    ///     Some(include_str!("../appCertPublicKey_2021002199679230.crt")),
    ///     Some(include_str!("../alipayRootCert.crt"))
    /// );
    /// let mut cli = config.get_client();
    pub fn new<'a>(
        app_id: &'a str,
        public_key: &'a str,
        private_key: &'a str,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Config {
        let app_cert_sn = app_cert_sn.map(|cert_sn| {
            app_cert_client::get_cert_sn_from_content(cert_sn.as_ref())
                .unwrap_or_else(|_| String::from(""))
        });
        let alipay_root_cert_sn = alipay_root_cert_sn.map(|root_cert_sn| {
            app_cert_client::get_root_cert_sn_from_content(root_cert_sn)
                .unwrap_or_else(|_| String::from(""))
        });
        Self {
            app_cert_sn,
            private_key: private_key.to_owned(),
            public_key: public_key.to_owned(),
            alipay_root_cert_sn,
            app_id: app_id.to_owned(),
        }
    }

    /// 创建client
    pub fn get_client(&self) -> Client {
        let mut params: HashMap<String, String> = HashMap::from([
            ("app_id".to_owned(), self.app_id.clone()),
            ("charset".to_owned(), "utf-8".to_owned()),
            ("sign_type".to_owned(), "RSA2".to_owned()),
            ("format".to_owned(), "json".to_owned()),
            ("version".to_owned(), "1.0".to_owned()),
        ]);

        if let Some(cert_sn) = self.app_cert_sn.clone() {
            params.insert("app_cert_sn".to_owned(), cert_sn);
        }
        if let Some(root_cert_sn) = self.alipay_root_cert_sn.clone() {
            params.insert("alipay_root_cert_sn".to_owned(), root_cert_sn);
        }
        Client::new_from_config(params, self.public_key.clone(), self.private_key.clone())
    }
}
