use alipay::param::{AlipayParam, FieldValue};
use chrono::Local;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct QueryParam {
    operation: String,
    page_num: i32,
    page_size: i32,
    item_id_list: Option<String>
}
async fn query() {
    let query = QueryParam {
        operation: "ITEM_PAGEQUERY".to_owned(),
        page_num: 1,
        page_size: 10,
        item_id_list: None,
    };
    let client = alipay::Client::new(
        "2021002199679230",
        "MIIEpQIBAAKCAQEAhyfuoXEgS4YtDmg7fk7w4/tCrTg8WVTLqT53qIvEoboinVVYJFQZASDcrl/KsgtOhkasuIGzwDadNoNEpE8pyaTv0dZpRfMDR4gZJ7tLnxpoa+nst9KAPCb8QX5RwRsL/c6Jfa4CFYj9DGb+putIIeB1fOewAhRiftcnGbZXBKrSXwq6DcpsE24mehz9/ddvGsBYUMWUvEQUKjPNC2wcxD7YoOZs7lMMGULoJNHUFuUTZZndXCQcN1CliQ7xHwjiUZNLGQhmLPhH/gKb3FCj7dGymsBNqR18H/LcsdCZkD2NLUuginqt5XxcM+n7HoIcPaAa+tSa6EYh52dMonPw5QIDAQABAoIBAGeKAdePsGvrKE0nMJx8oTIl5FiLAkB1I2hOQKDQIhy7WZUqMlHyUw14PVcgb0miO8/GCL94LVoM/LcsLMOrGZouTsJz/UXm+xYrfwnfA/mo42H4XK4eBrsOKqWJvduveqo/NTkgutwAi8qahG8fQ60gJSFA5KdTMnl1HbEm7NbXYGwwvIYhD2PXxOZ2BjyW82p9/uh7RBfGRyHXwT9V52WQgX05rwSYImAzunWOSoo/B8KDFJbQzlIdnXM5nvfcoeyseOxaUGZaWTbqtU539tbDDiwjkaRkOwszf47/Oud9VkpohAuiduv+T0SBNVkeA+z+Ehfo+5ZyHsTE/elMl8ECgYEAyKxZT1ikJ8tjwmTtmQafKBogyAOa0iJZovNwqD5IsE2vqPO70YpQUXt6IOZLZbkjjFGrWCudlf8jEzuO1iHb82jpECjFKBHeFndOjBJxvjV+8O/Vxr9roJaT0vxpzWkcmU1/MRkKbAU061xX2j/k4Iz7kQlp+HqAOLjwVvk6Ay0CgYEArGtU8rQZ6MKVd570hy/fr/u7NAmvoxBSnCrOQ6JsYFcmaCJDj2q0cb1lw0ASRozxuSmZIH7Ee0B9XNxrcX2sEpW+PaBn+M6iTuPSUWV5ql0x0oymWsa0xFGV45YEsYB41wkd38XtG8Dxz+SxdRU3qobZ9eYkbIZxiWhevGXbF5kCgYEAtQlneQHK9muzEAjloQwsQY1wzYETB0gd/bgJhn7KLOOo+Y8Jfjx9wUTYJR4eHyMrQsfbAKw4er218v/kGKJrP+kBeaaOV1vnM/VmU0/AdYzlfI+iGK9QdYviyJEXEk0lk9gqSy0ADfuUhlDEoQzLexk1St9nTteVHZcanBwzjfECgYEAppQoBThVk9hy+ZgcHYP2NBscGUGGbB94AKMmlpeU51srow3/gc8QuJbIe2Qqg/jmDQOQiqGPCJkcxRu7vnExTt9XZkjUSsCwdVGMP9GvQxY47XevvSIfQVClZLTqoedCWFbZgvnBg7/coAMOI9U0687PQ9Buvl8B0ESCyrgJfXkCgYEAluzJf0ax/DVwx8IaY+ZusdRFixv1uxazWYVf8yFrFot+hWJPQI/Bw20JtZRlQ+Ur7y/w+LyxHNPcMTkg8m0N5dWn5Mykcsmd6YLBwvbNasGM/v3/d9mHgb4XBc/u+pc3b7bvhO5CSMDcVCX+Vk+/zH6Q1BXVpxWYLB/G4bIkIAY=",
        Some("appCertPublicKey_2021002199679230.crt"),
        Some("alipayRootCert.crt")
    );

    let data:serde_json::Value = client
        .post("alipay.open.mini.item.page.query", query)
        .await.unwrap();
    println!("{:?}", data);
}

#[tokio::main]
async fn main() {
    query().await;
}