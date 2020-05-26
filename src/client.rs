use anyhow::Error;
use isahc::{prelude::*, HttpClient};
use std::time::Duration;
use http::Uri;

pub enum Endpoints {
    Login,
    LoginAction,
    LogoutAction,
    PubIp,
    Saldo,
}

// get endpoint url from Endpoints enum
fn endpoint_url(e: Endpoints) -> Uri {
    let base_url = "https://m.klikbca.com/";
    match e {
        Endpoints::Login => [base_url, "login.jsp"].concat().parse::<Uri>().unwrap(),
        Endpoints::LoginAction => [base_url, "authentication.do"].concat().parse::<Uri>().unwrap(),
        Endpoints::LogoutAction => [base_url, "authentication.do?value=(actions)=logout"].concat()
            .parse::<Uri>().unwrap(),
        Endpoints::Saldo => [base_url, "balanceinquiry.do"].concat().parse::<Uri>().unwrap(),
        Endpoints::PubIp => "https://icanhazip.com".parse::<Uri>().unwrap(),
    }
}

// build http client with default headers and cookies support
fn build_headers() -> Result<HttpClient, Error> {
    let clientb = HttpClient::builder()
        .timeout(Duration::from_secs(7))
        .cookies()
        .default_headers(&[
                         ("Host", "m.klikbca.com"),
                         ("Connection", "keepalive"),
                         ("Cache-Control", "max-age=0"),
                         ("Upgrade-Insecure-Request", "1"),
                         ("User-Agent","Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2490.76 Mobile Safari/537.36"),
                         ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
                         ("Accept-Encoding", "gzip, deflate, sdch, br"),
                         ("Accept-Language", "en-US,en;q=0.8,id;q=0.6,fr;q=0.4"),
        ]);
    Ok(clientb.build()?)
}



// Client is the reusable http client for the entire program.
// consists of the reusable http client itself, the uri (mutable), and the body (to be used for
// POST, PUT methods).
#[derive(Debug)]
pub struct Client {
    // hc is isahc::HttpClient
    hc: HttpClient,
    // uri -> url for operation (this is mutable)
    uri: Uri,
}

impl Client {
    // Create default Client struct
    pub fn new() -> Result<Self, Error> {
        let client = build_headers()?;
        Ok(Client{
            hc: client,
            uri: "https://m.klikbca.com".parse::<Uri>().unwrap(),
        })
    }
    // Client.set_url() sets the current url to the specified Endpoint
    pub fn set_url(&mut self, ep: Endpoints) -> &Self {
        self.uri = endpoint_url(ep);
        self
    }

    // Gets current public ip
    pub async fn get_pub_ip(&mut self) -> Result<String, Error> {
        self.set_url(Endpoints::PubIp);
        let pub_ip_string = self.get().await?;
        let ip = pub_ip_string.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        // Ok(Box::leak(ip.into_boxed_str()))
        Ok(ip)
    }

    // Async GET 
    pub async fn get(&self) -> Result<String, Error> {
        let mut resp = self.hc.get_async(&self.uri).await?;
        Ok(resp.text_async().await?)
    }

    // Async POST, takes body (can be a string literal)
    pub async fn post(&self, body: String) -> Result<String, Error> {
        let mut resp = self.hc.post_async(&self.uri, body).await?;
        Ok(resp.text_async().await?)
    }
}
