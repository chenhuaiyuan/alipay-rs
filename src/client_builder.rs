use crate::Client;

#[derive(Default)]
pub struct ClientBuilder<'a> {
    app_id: Option<&'a str>,
    public_key: Option<&'a str>,
    private_key: Option<&'a str>,
    app_cert_sn: Option<&'a str>,
    alipay_root_cert_sn: Option<&'a str>,
    alipay_public_key: Option<&'a str>,
    sandbox: bool,
}

impl<'a> ClientBuilder<'a> {
    /// 添加app_id
    pub fn app_id(&mut self, app_id: &'a str) -> &mut Self {
        self.app_id = Some(app_id);
        self
    }
    /// 添加public_key
    pub fn public_key(&mut self, public_key: &'a str) -> &mut Self {
        self.public_key = Some(public_key);
        self
    }
    /// 添加private_key
    pub fn private_key(&mut self, private_key: &'a str) -> &mut Self {
        self.private_key = Some(private_key);
        self
    }
    // 添加app_cert_sn
    pub fn app_cert_sn(&mut self, app_cert_sn: &'a str) -> &mut Self {
        self.app_cert_sn = Some(app_cert_sn);
        self
    }
    /// 添加alipay_root_cert_sn
    pub fn alipay_root_cert_sn(&mut self, alipay_root_cert_sn: &'a str) -> &mut Self {
        self.alipay_root_cert_sn = Some(alipay_root_cert_sn);
        self
    }
    /// 调用此函数将调用沙箱环境
    pub fn sandbox(&mut self) -> &mut Self {
        self.sandbox = true;
        self
    }
    /// 添加支付宝公钥，目前没有任何用途，可以忽略
    pub fn alipay_public_key(&mut self, alipay_public_key: &'a str) -> &mut Self {
        self.alipay_public_key = Some(alipay_public_key);
        self
    }
    pub fn finish(&self) -> Client {
        Client::new(
            self.app_id.unwrap_or(""),
            self.public_key.unwrap_or(""),
            self.private_key.unwrap_or(""),
            self.app_cert_sn,
            self.alipay_root_cert_sn,
            self.sandbox,
        )
    }
}
