use super::Merchant;
use crate::error::AlipayResult;
use serde::{de::DeserializeOwned, Serialize};


impl Merchant {
    pub async fn merchant_expand_item_open_create<T: Serialize, R: DeserializeOwned>(
        self,
        params: T,
    ) -> AlipayResult<R> {
        let data: R = self
            .client
            .post("ant.merchant.expand.item.open.create", params)
            .await?;
        Ok(data)
    }
}