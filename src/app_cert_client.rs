use crate::error::AlipayResult;
use md5::{Digest, Md5};
use openssl::x509::{X509NameEntries, X509};
use std::fs;

// 从证书中获取序列号
pub fn get_cert_sn(cert_path: &str) -> AlipayResult<String> {
    let cert = fs::read(cert_path)?;
    get_cert_sn_from_content(cert.as_slice())
}

pub fn get_cert_sn_from_content(cert_content: &[u8]) -> AlipayResult<String> {
    let ssl = X509::from_pem(cert_content)?;
    let issuer = iter2string(ssl.issuer_name().entries())?;
    let serial_number = ssl.serial_number().to_bn()?.to_string();
    let mut hasher = Md5::new();
    hasher.update((issuer + &serial_number).as_bytes());
    let res = hasher.finalize();
    Ok(hex::encode(&res[..]))
}
// 提取根证书序列号
pub fn get_root_cert_sn_from_content(cert_content: String) -> AlipayResult<String> {
    let certificate_end = "-----END CERTIFICATE-----";
    let mut array: Vec<&str> = cert_content.split(certificate_end).collect();
    let mut i = 0;
    while i < array.len() {
        if array[i].is_empty() {
            array.remove(i);
        } else {
            i += 1;
        }
    }
    let mut sn: String = String::new();
    for cert in array {
        let c = cert.to_string() + certificate_end;
        let ssl = X509::from_pem(c.as_bytes())?;
        let issuer = iter2string(ssl.issuer_name().entries())?;
        let serial_number = ssl.serial_number().to_bn()?.to_string();
        let mut hasher = Md5::new();
        hasher.update((issuer + &serial_number).as_bytes());
        let res = hasher.finalize();
        let res = hex::encode(&res[..]);
        if sn.is_empty() {
            sn = res;
        } else {
            sn = sn + "_" + &res;
        }
    }
    Ok(sn)
}
pub fn get_root_cert_sn(cert_path: &str) -> AlipayResult<String> {
    let cert_content = fs::read_to_string(cert_path)?;
    get_root_cert_sn_from_content(cert_content)
}
fn iter2string(iter: X509NameEntries) -> AlipayResult<String> {
    let mut string: String = String::from("");
    for value in iter {
        let data = value.data().as_utf8()?.to_string();
        let key = value.object().to_string();
        string.push_str(&(key + "=" + &data + ","));
    }
    string.remove(string.len() - 1);
    Ok(string)
}
