use anyhow::{anyhow, Result};
use isahc::prelude::*;
use url::{form_urlencoded, Url};

pub enum Endpoints {
    Login,
    Authentication,
    AccountStatement,
    BalanceInquiry,
    PubIp,
}

// creates & validates url from string literal
pub fn build_url<I>(base: &str, path: I) -> Result<Url>
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
        Endpoints::Login => build_url(base_url, &["login.jsp"]),
        Endpoints::Authentication => build_url(base_url, &["authentication.do"]),
        Endpoints::AccountStatement => build_url(base_url, &["accountstmt.do"]),
        Endpoints::BalanceInquiry => build_url(base_url, &["balanceinquiry.do"]),
        Endpoints::PubIp => build_url("http://icanhazip.com", &[""]),
    }
}

// produces default headers for the reusable http client.
fn default_headermap() -> http::HeaderMap {
    let mut new_headers = http::HeaderMap::new();
    new_headers
        .insert(
            http::header::USER_AGENT,
            "Mozilla/5.0 (iPhone; CPU OS 10_15_4 Supplemental Update like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1.1 Mobile/14E304 Safari/605.1.15".parse().unwrap(),
            );

    new_headers.insert(
        http::header::UPGRADE_INSECURE_REQUESTS,
        "1".parse().unwrap(),
    );

    new_headers.insert(
        http::header::ORIGIN,
        "https://m.klikbca.com".parse().unwrap(),
    );

    new_headers.insert(http::header::HOST, "m.klikbca.com".parse().unwrap());

    new_headers.insert(
        http::header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            .parse()
            .unwrap(),
    );

    new_headers.insert(
        http::header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.5".parse().unwrap(),
    );

    new_headers.insert(
        http::header::ACCEPT_ENCODING,
        "gzip, deflate, br".parse().unwrap(),
    );

    new_headers
}

// Req is the reusable http client for the entire program.
// consists of the reusable http client itself, the uri (mutable), and the body (to be used for
// POST, PUT methods).
#[derive(Debug)]
pub struct Client {
    c: HttpClient,
}

impl Client {
    // Create default Client struct
    pub fn new() -> Result<Self> {
        let c = HttpClient::builder()
            .redirect_policy(isahc::config::RedirectPolicy::Limit(10))
            .default_headers(&default_headermap())
            .cookies()
            .tcp_keepalive(std::time::Duration::from_secs(300))
            .auto_referer()
            .build()?;
        Ok(Client { c })
    }

    pub fn simple_get(&self, u: &Url) -> Result<String> {
        let req = Request::get(u.as_str()).body(Body::empty())?;
        let mut resp = self.c.send(req)?;
        Ok(resp.text()?)
    }

    pub fn get(&mut self, u: &Url) -> Result<String> {
        let req = Request::get(u.as_str())
            .redirect_policy(isahc::config::RedirectPolicy::Limit(2))
            .tcp_keepalive(std::time::Duration::from_secs(3600))
            .body(Body::empty())?;
        let mut resp = self.c.send(req)?;
        Ok(resp.text()?)
    }

    pub fn post<I, K, V>(&mut self, u: &Url, form: Option<I>) -> Result<String>
    where
        I: IntoIterator,
        I::Item: core::borrow::Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let req: Request<Body>;
        match form {
            Some(f) => {
                let mut form_data = form_urlencoded::Serializer::new(String::new());
                form_data.extend_pairs(f);
                let form_string: String = form_data.finish().into();
                req = Request::post(u.as_str())
                    .redirect_policy(isahc::config::RedirectPolicy::Limit(5))
                    .tcp_keepalive(std::time::Duration::from_secs(3600))
                    .body(Body::from_bytes(form_string.into_bytes()))?;
            }
            None => {
                req = Request::post(u.as_str())
                    .redirect_policy(isahc::config::RedirectPolicy::Limit(5))
                    .tcp_keepalive(std::time::Duration::from_secs(3600))
                    .body(Body::empty())?;
            }
        }
        let mut resp = self.c.send(req)?;
        Ok(resp.text()?)
    }
}
