use std::{u8, fs};
use rocket::http::Status;
use urlencoding::encode;
use hyper::{body::HttpBody as _, Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use serde_json::Value;
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
async fn get_json(q: &str, to: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let request = hyper::Request::builder()
        .uri(format!("http://translate.google.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&dt=bd&ie=UTF-8&oe=UTF-8&dj=1&source=icon&q={}", to, q))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36")
        .body(Body::from(""))?;
    let https = HttpsConnector::new();
    let http_connector: hyper_proxy::ProxyConnector<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;
    let proxy = Proxy::new(Intercept::All, "http://127.0.0.1:10809".parse::<Uri>()?);
    http_connector = ProxyConnector::from_proxy(https, proxy)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(http_connector);
    let mut res = client.request(request).await?;
    let mut body: Vec<u8> = vec![];
    while let Some(chunk) = res.body_mut().data().await {
        let bt = chunk?;
        for b in bt.iter() {
            body.push(*b)
        }
    }
    Ok(body)
}
#[get("/api/trans?<q>&<to>")]
pub async fn trans(q: String, to: String) -> Result<Vec<u8>, Status> {
    match get_json(encode(q.as_str()).to_string().as_str(), to.as_str()).await {
        Ok(v) => Ok(v),
        Err(v) => Err(Status::NotFound)
    }
}