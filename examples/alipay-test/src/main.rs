use serde::Serialize;
use chrono::{Local};
use alipay_rs::param::{AlipayParam, FieldValue};

#[derive(Serialize, Debug)]
struct Transfer {
    out_biz_no: String,
    trans_amount: String,
    product_code: String,
    biz_scene: String,
    payee_info: PayeeInfo,
}
#[derive(Serialize, Debug)]
struct PayeeInfo {
    identity: String,
    identity_type: String,
    name: String,
}
async fn naive_fund_transfer() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let client = alipay_rs::Client::new(
        "2021002182623971",
        "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
        Some("appCertPublicKey_2021002182623971.crt"),
        Some("alipayRootCert.crt")
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
#[derive(Serialize, Debug)]
struct StationQuery {
    city_code: String
}
async fn naive_station_query() {
    let station_query = StationQuery {
        city_code: String::from("330100")
    };
    let client = alipay_rs::Client::new(
        "2021002182623971",
        "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
        None,
        None,
    );
    let data:serde_json::Value = client
        .post("alipay.commerce.cityfacilitator.station.query", station_query)
        .await.unwrap();
    println!("{:?}", data);
}
async fn neo_fund_transfer() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let client = alipay_rs::Client::neo(
        "2021002182623971",
        "私钥.txt",
        Some("appCertPublicKey_2021002182623971.crt"),
        Some("alipayRootCert.crt")
    );
    let data:serde_json::Value = client
        .post("alipay.fund.trans.uni.transfer", transfer)
        .await.unwrap();
    println!("{:?}", data);
}
async fn fund_transfer() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let client = alipay_rs::Client::new(
        "2021002182623971",
        "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
        Some("appCertPublicKey_2021002182623971.crt"),
        Some("alipayRootCert.crt")
    );
    let api = alipay_rs::api::Fund::new(client);
    let data: serde_json::Value = api.fund_trans_uni_transfer(transfer).await.unwrap();
    println!("{:?}", data);
}

#[derive(AlipayParam)]
struct PublicParams {
    app_id: String,
    method: Option<String>,
    charset: String,
    sign_type: String,
    sign: Option<String>,
    timestamp: Option<String>,
    version: String,
    biz_content: Option<String>,
}
async fn fund_transfer_from_public_params() {
    let transfer = Transfer {
        out_biz_no: format!("{}", Local::now().timestamp()),
        trans_amount: String::from("0.1"),
        product_code: String::from("TRANS_ACCOUNT_NO_PWD"),
        biz_scene: String::from("DIRECT_TRANSFER"),
        payee_info: PayeeInfo {
            identity: String::from("343938938@qq.com"),
            identity_type: String::from("ALIPAY_LOGON_ID"),
            name: String::from("陈怀远"),
        },
    };
    let mut client = alipay_rs::Client::new(
        "2021002182623971",
        "MIIEoQIBAAKCAQEArcZcqObeVuMgKNZI6RrJf9gEP5Al4ju9C37dm4iMsfZ9GdR7xP8m24KAJH8mko3+ZNsa3HeEFTQeXtOfhx+tQmlWVG+lj04aVWRzCA5UjFeDrMkFIRTf0x/gR/aBq2W9JS8yR1taQ+OKrNFn9OTeNZMv0nUUgypF7adAse9T6pKBRVGe+3N4yCOUg8GsjrcVv7u0pUxAcU4Erytxo9BMBNVeFNsA/fNujUT08lUDo6i4AH37yEZgQSbL4Hh+rUpKL/9EXoLpOZPR0NOEnxE1fuRRnkYS4dSkgPlww3+V7MoFVx65TDvakpchzJOKGa/QCEhxkHuI4nLjm9PgRAls3QIDAQABAoH/MN+ZL+e+oLFAWjvqRaVDBrG6gCYKgZZLlPAZY6UD7QlmJd2c8crRIuuRHrKkJpPI+JSm+Vqjy1LdN85ND7PZBtSZcyXzalqNDXcy4xEktlPmtLHUv3kfekF80sCBt7Llf4/GlEsdF/rnBbPfiQDVfjvnN0m2ey1ofW6Mw36MG2ygerQs0lnE924RjnDyvMsTP4qbIroHkT+TLHtBf14nxQadEX/0bfUY7yqTswqqul3j5sSJZTQIk1eCzaYP1iollRj3MGKJ7XTiIOEkj7+zT3cDo/DUlSs3EkuBER1EtM42g6MD4WfJ3yr+VT9BeWJGJJyJm4kV28mRC7wVgZABAoGBAPsl5r+MtvSbhM+1wtjWl/bQzSpG4DkZesZELjyCkRagC9M+EHSq+aqqyVjnMIeY9pptD/6tsHfxMD/4SRqTMQ2A26zDpM36Trw3u8777rTEq/8Sbl3PFGBgczZTtSkd4pQwtwV8jwjKoLJcuKdkPQpxpsRfnp7O45JOwu6D90ddAoGBALEhzBoCM022k/ovvQhq0ZCQS4DZrv8vudlckQNtQHFZefUruLAhgo7vxHVo8WeHBUAtiOAUZikZS5KAgaXuoGhADE95nxMGZcG9fdsuL8su9ysPjuwZ3W3wfRIKCTurFfORmydOLf8Ej82n43V6SQAo0QjbRR4CPAc6N5gBU+OBAoGABGc0tXUFHCLB4FZidSTGA0jD4BLgCYA9284ENYFgg9IIgwqahUEeIXTfFNTwz9/Jqwlwd1maN3AeFXEH7xRXjtIMh+niMM5LpRchDs7x729nSJCNKM3hoJLwUiqDiZYBi/GSs+DsLQ5IZPglMKIcQ9ucPeMjR8t+x+jjmATuR+0CgYAwj5J0AuxrvsU8zr+lQhun5Vc9wPAP99act5rt9JK5QI2F4HGmn9k6NJOImLet6T9QQ+uFezIyzEOCq4ZfplcFnaGCXFZ3Ecbt4XRSlYv2yS5r+Lz3D3Q8QrUXL/cuC45eEyoVEYLcqjR+biuWtmqzB32fTvXY70XjuVsqahrEgQKBgQDnvO2QZmosVy8KycqmsOgGdQJ35SWrfR2D9evrGLEy/+tJhzLGYDEQWW96crWWjFHwBCRltmUNcz3i3qB0yblNoGpJB4VDvz3MkpVu++ZxiIDxA8J+A7Q2s9klGi29e3vej5XZCp3BVyVPfAVgXkBYlMTc1rXr0FUVKGMjnm6d4A==",
        Some("appCertPublicKey_2021002182623971.crt"),
        Some("alipayRootCert.crt")
    );
    let public_params = PublicParams {
        app_id: "2021002182623971".to_owned(),
        method: None,
        charset: "utf-8".to_owned(),
        sign_type: "RSA2".to_owned(),
        sign: None,
        timestamp: None,
        version: "1.0".to_owned(),
        biz_content: None,
    };
    client.set_public_params(public_params);
    let api = alipay_rs::api::Fund::new(client);
    let data: serde_json::Value = api.fund_trans_uni_transfer(transfer).await.unwrap();
    println!("{:?}", data);
}

#[derive(AlipayParam)]
struct ImageUpload {
    image_type: String,
    image_name: String,
}

async fn image_upload() {
    let file = std::fs::read("./test.png").unwrap();
    let image = ImageUpload {
        image_type: "png".to_owned(),
        image_name: "test".to_owned(),
    };
    let mut client = alipay_rs::Client::new(
        "2021002199679230",
        "MIIEpQIBAAKCAQEAhyfuoXEgS4YtDmg7fk7w4/tCrTg8WVTLqT53qIvEoboinVVYJFQZASDcrl/KsgtOhkasuIGzwDadNoNEpE8pyaTv0dZpRfMDR4gZJ7tLnxpoa+nst9KAPCb8QX5RwRsL/c6Jfa4CFYj9DGb+putIIeB1fOewAhRiftcnGbZXBKrSXwq6DcpsE24mehz9/ddvGsBYUMWUvEQUKjPNC2wcxD7YoOZs7lMMGULoJNHUFuUTZZndXCQcN1CliQ7xHwjiUZNLGQhmLPhH/gKb3FCj7dGymsBNqR18H/LcsdCZkD2NLUuginqt5XxcM+n7HoIcPaAa+tSa6EYh52dMonPw5QIDAQABAoIBAGeKAdePsGvrKE0nMJx8oTIl5FiLAkB1I2hOQKDQIhy7WZUqMlHyUw14PVcgb0miO8/GCL94LVoM/LcsLMOrGZouTsJz/UXm+xYrfwnfA/mo42H4XK4eBrsOKqWJvduveqo/NTkgutwAi8qahG8fQ60gJSFA5KdTMnl1HbEm7NbXYGwwvIYhD2PXxOZ2BjyW82p9/uh7RBfGRyHXwT9V52WQgX05rwSYImAzunWOSoo/B8KDFJbQzlIdnXM5nvfcoeyseOxaUGZaWTbqtU539tbDDiwjkaRkOwszf47/Oud9VkpohAuiduv+T0SBNVkeA+z+Ehfo+5ZyHsTE/elMl8ECgYEAyKxZT1ikJ8tjwmTtmQafKBogyAOa0iJZovNwqD5IsE2vqPO70YpQUXt6IOZLZbkjjFGrWCudlf8jEzuO1iHb82jpECjFKBHeFndOjBJxvjV+8O/Vxr9roJaT0vxpzWkcmU1/MRkKbAU061xX2j/k4Iz7kQlp+HqAOLjwVvk6Ay0CgYEArGtU8rQZ6MKVd570hy/fr/u7NAmvoxBSnCrOQ6JsYFcmaCJDj2q0cb1lw0ASRozxuSmZIH7Ee0B9XNxrcX2sEpW+PaBn+M6iTuPSUWV5ql0x0oymWsa0xFGV45YEsYB41wkd38XtG8Dxz+SxdRU3qobZ9eYkbIZxiWhevGXbF5kCgYEAtQlneQHK9muzEAjloQwsQY1wzYETB0gd/bgJhn7KLOOo+Y8Jfjx9wUTYJR4eHyMrQsfbAKw4er218v/kGKJrP+kBeaaOV1vnM/VmU0/AdYzlfI+iGK9QdYviyJEXEk0lk9gqSy0ADfuUhlDEoQzLexk1St9nTteVHZcanBwzjfECgYEAppQoBThVk9hy+ZgcHYP2NBscGUGGbB94AKMmlpeU51srow3/gc8QuJbIe2Qqg/jmDQOQiqGPCJkcxRu7vnExTt9XZkjUSsCwdVGMP9GvQxY47XevvSIfQVClZLTqoedCWFbZgvnBg7/coAMOI9U0687PQ9Buvl8B0ESCyrgJfXkCgYEAluzJf0ax/DVwx8IaY+ZusdRFixv1uxazWYVf8yFrFot+hWJPQI/Bw20JtZRlQ+Ur7y/w+LyxHNPcMTkg8m0N5dWn5Mykcsmd6YLBwvbNasGM/v3/d9mHgb4XBc/u+pc3b7bvhO5CSMDcVCX+Vk+/zH6Q1BXVpxWYLB/G4bIkIAY=",
        Some("appCertPublicKey_2021002199679230.crt"),
        Some("alipayRootCert.crt")
    );
    client.set_public_params(image);
    // let b: &'static [u8] = file.as_ref();
    let data:serde_json::Value = client.post_file("alipay.offline.material.image.upload", "image_content", "test.png", file.as_ref()).await.unwrap();
    println!("{:?}", data);
}
#[tokio::main]
async fn main() {
    // naive_fund_transfer().await;
    // fund_transfer().await;
    neo_fund_transfer().await;
    // fund_transfer_from_public_params().await;
    // image_upload().await;
}
