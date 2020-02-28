use hyper::client::{Client, HttpConnector};
use hyper::Body;
use hyper_tls::native_tls;
use hyper_tls::HttpsConnector;
use tokio_tls::TlsConnector;
use http::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_tls_connector = native_tls::TlsConnector::new()?;
    let tls_connector = TlsConnector::from(native_tls_connector);
    let mut http_connector = HttpConnector::new();
    http_connector.enforce_http(false);
    let mut https_connector = HttpsConnector::from((http_connector, tls_connector));
    https_connector.https_only(true);
    let client = Client::builder().build::<_, Body>(https_connector);
    let resp = client
        .get("https://github.com".parse()?)
        .await?;
    assert_eq!(StatusCode::OK, resp.status());
    Ok(())
}