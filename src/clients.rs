use anyhow::{anyhow, Result};
use isahc::prelude::*;
use isahc::{HttpClient, RequestExt};
use url::{Url, form_urlencoded};

pub enum Endpoints {
    // Login,
    LoginAction,
    LogoutAction,
    PubIp,
    Saldo,
}

fn build_url<I>(base: &str, path: I) -> Result<Url>
where
I: IntoIterator,
I::Item: AsRef<str>,
{
    let mut url = Url::parse(base)?;
    url.path_segments_mut()
        .map_err(|_| anyhow!("cannot be base"))?
        .extend(path);
    Ok(url)
}

// get endpoint url from Endpoints enum
pub fn endpoint_url(e: Endpoints) -> Result<Url> {
    let base_url = "https://m.klikbca.com/";
    match e {
        Endpoints::LoginAction => build_url(base_url, &["authentication.do"]),
        Endpoints::LogoutAction => build_url(base_url,
                                             &["authentication.do?value=(actions)=logout"]),
        Endpoints::Saldo => build_url(base_url, &["balanceinquiry.do"]),
        Endpoints::PubIp => build_url("http://icanhazip.com", &[""]),
    }
}

// build http client with default headers and cookies support
fn default_headers() -> http::HeaderMap {
    let mut default_headers = http::HeaderMap::new();
    default_headers.insert(
        http::header::UPGRADE_INSECURE_REQUESTS,
        "1".parse().expect("Invalid insecure request upgrade"),
        );
    default_headers.insert(
        http::header::USER_AGENT,
        "Mozilla/5.0 (Linux; Android 9; 5032W) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.136 Mobile Safari/537.36"
        .parse().expect("Invalid UA"),
        );
    default_headers
}

// Req is the reusable http client for the entire program.
// consists of the reusable http client itself, the uri (mutable), and the body (to be used for
// POST, PUT methods).
#[derive(Debug)]
pub struct Client {
    // hc is isahc::Client
    hc: HttpClient,
}

impl Client {
    // Create default Client struct
    pub fn new() -> Result<Self> {
        let client = HttpClient::builder()
            .cookies()
            .default_headers(&default_headers())
            .build()?;
        Ok(Client {
            hc: client,
        })
    }

    pub async fn get(&self, u: &Url) -> Result<String> {
        let mut resp = self.hc.get_async(u.as_str()).await?;
        println!("Response headers: {:?}", resp.headers());
        Ok(resp.text_async().await?)
    }

    pub async fn post(&self, u: Url) -> Result<String> {

    }
    // Gets current public ip
    // pub async fn get_pub_ip(&mut self) -> Result<String, Error> {
    //     self.set_url(Endpoints::PubIp);
    //     let pub_ip_string = self.get().await?;
    //     let ip = pub_ip_string
    //         .chars()
    //         .filter(|c| !c.is_whitespace())
    //         .collect::<String>();
    //     Ok(ip)
    // }

    // Async GET
    // pub async fn get(&self) -> Result<String, Error> {
    //     let url: String = self.get_uri_string();
    //     let get_url = url.as_str();
    //     let resp = self.hc.get(get_url).send().await?;
    //     Ok(resp.text().await?)
    // }

    // Async POST, can take optional form
    // pub async fn post(&self, form: Option<Vec<(&str, &str)>>) -> Result<String, Error> {
    //     let url: String = self.get_uri_string();
    //     let post_url = url.as_str();
    //     let resp: Response;
    //     match form {
    //         Some(f) => resp = self.hc.post(post_url).form(&f).send().await?,
    //         None => resp = self.hc.post(post_url).send().await?,
    //     }
    //     Ok(resp.text().await?)
    // }
}
