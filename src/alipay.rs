use serde::{Deserialize, Serialize};

pub struct RequestParams {
    app_id: String,
    method: String,
    charset: String,
    sign_type: String,
    sign: Option<String>,
    timestamp: String,
    version: String,
    biz_content: Option<Serialize>,
}

pub struct Client {
    api_url: &str,
    request_params: RequestParams,
}
impl Client {}
