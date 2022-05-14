use crate::error::AlipayResult;
use openssl::{
    hash::{hash, MessageDigest},
    nid::Nid,
    x509::{X509NameEntries, X509},
};
use std::fs;

pub(crate) fn get_private_key_from_file(key_path: &str) -> AlipayResult<String> {
    let private_key = fs::read_to_string(key_path)?;
    Ok(private_key)
}

// 从证书中获取序列号
pub(crate) fn get_cert_sn(cert_path: &str) -> AlipayResult<String> {
    let cert = fs::read_to_string(cert_path)?;
    // let ssl = X509::from_pem(cert.as_slice())?;
    get_cert_sn_from_content(cert)
}

pub(crate) fn get_cert_sn_from_content(content: String) -> AlipayResult<String> {
    let ssl = X509::from_pem(content.as_bytes())?;
    let issuer = iter2string(ssl.issuer_name().entries())?;
    let serial_number = ssl.serial_number().to_bn()?.to_dec_str()?;
    let data = issuer + &serial_number;
    Ok(hex::encode(hash(MessageDigest::md5(), data.as_ref())?))
}
// 提取根证书序列号
pub(crate) fn get_root_cert_sn_from_content(cert_content: String) -> AlipayResult<String> {
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
        if ssl.signature_algorithm().object().nid() == Nid::SHA256WITHRSAENCRYPTION
            || ssl.signature_algorithm().object().nid() == Nid::SHA1WITHRSAENCRYPTION
        {
            let res = get_cert_sn_from_content(c)?;
            if sn.is_empty() {
                sn = res;
            } else {
                sn = sn + "_" + &res;
            }
        }
    }
    Ok(sn)
}
pub(crate) fn get_root_cert_sn(cert_path: &str) -> AlipayResult<String> {
    let cert_content = fs::read_to_string(cert_path)?;
    get_root_cert_sn_from_content(cert_content)
}

fn iter2string(iter: X509NameEntries) -> AlipayResult<String> {
    let mut string: String = String::from("");
    for value in iter {
        let data = value.data().as_utf8()?.to_string();
        let key = value.object().nid().short_name()?.to_owned();
        string.insert_str(0, &(key + "=" + &data + ","));
    }
    string.pop();
    Ok(string)
}
