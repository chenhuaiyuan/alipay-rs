use crate::alipay::Client;
use crate::error::AlipayResult;
use serde::{de::DeserializeOwned, Serialize};

pub async fn fund_trans_uni_transfer<T: Serialize, R: DeserializeOwned>(
    client: Client,
    params: T,
) -> AlipayResult<R> {
    let data: R = client
        .post("alipay.fund.trans.uni.transfer", params)
        .await?;
    Ok(data)
}
