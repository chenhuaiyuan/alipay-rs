use crate::{app_cert_client, error::AlipayResult, Client};
use openssl::{
    base64,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::{Signer, Verifier},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct ConfigBuilder<'a> {
    app_id: Option<&'a str>,
    public_key: Option<&'a str>,
    private_key: Option<&'a str>,
    app_cert_sn: Option<&'a str>,
    alipay_root_cert_sn: Option<&'a str>,
}

#[derive(Debug)]
pub struct Config<'a> {
    app_id: &'a str,
    public_key: &'a str,
    private_key: &'a str,
    app_cert_sn: Option<String>,
    alipay_root_cert_sn: Option<String>,
}

impl<'a> Config<'a> {
    pub fn builder<'a>() -> ConfigBuilder<'a> {
        COnfigBuilder::default()
    }
    pub fn new(
        app_id: &'a str,
        public_key: &'a str,
        private_key: &'a str,
        app_cert_sn: Option<&str>,
        alipay_root_cert_sn: Option<&str>,
    ) -> Config<'a> {
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
            private_key,
            public_key,
            alipay_root_cert_sn,
            app_id,
        }
    }

    pub fn get_client(&self) -> Client {
        let mut params: HashMap<String, String> = HashMap::from([
            ("app_id".to_owned(), self.app_id.to_string()),
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
        Client::new_from_config(
            params,
            self.public_key.to_string(),
            self.private_key.to_string(),
        )
    }

    pub fn get_private_key(&self) -> AlipayResult<PKey<Private>> {
        let cert_content = base64::decode_block(self.private_key)?;
        let rsa = Rsa::private_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }

    pub fn get_public_key(&self) -> AlipayResult<PKey<Public>> {
        let cert_content = base64::decode_block(self.public_key)?;
        let rsa = Rsa::public_key_from_der(cert_content.as_slice())?;

        Ok(PKey::from_rsa(rsa)?)
    }
}
