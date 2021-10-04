mod app_cert_client;
mod error;

fn main() {
    // let data = app_cert_client::get_cert_sn("appCertPublicKey_2021002182623971.crt").unwrap();
    let data = app_cert_client::get_root_cert_sn("alipayRootCert.crt").unwrap();
    println!("{}", data)
}
