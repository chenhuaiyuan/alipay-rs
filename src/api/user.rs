use super::User;
use crate::{error::AlipayResult, param::AlipayParam};
use serde::de::DeserializeOwned;

impl User {
    /// 换取授权访问令牌
    /// 文档：<https://opendocs.alipay.com/open/284/web>
    pub async fn system_oauth_token<T: AlipayParam, R: DeserializeOwned>(
        mut self,
        params: T,
    ) -> AlipayResult<R> {
        self.client.add_public_params(params);
        let data: R = self
            .client
            .no_param_post("alipay.system.oauth.token")
            .await?;
        Ok(data)
    }
}
