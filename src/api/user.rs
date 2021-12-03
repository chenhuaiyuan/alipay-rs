use super::User;
use crate::error::AlipayResult;
use serde::{de::DeserializeOwned, Serialize};

impl User {
    /// 换取授权访问令牌
    /// 文档：<https://opendocs.alipay.com/open/284/web>
    pub async fn system_oauth_token<T: Serialize, R: DeserializeOwned>(
        self,
        params: T,
    ) -> AlipayResult<R> {
        let data: R = self
            .client
            .post("alipay.system.oauth.token", params)
            .await?;
        Ok(data)
    }
}
