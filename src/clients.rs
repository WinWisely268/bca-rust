use crate::cookies::{Cookie, CookieJar};
use anyhow::{anyhow, Result};
use http::header::SET_COOKIE;
use isahc::prelude::*;
use url::{form_urlencoded, Url};

pub enum Endpoints {
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
        Endpoints::LogoutAction => {
            build_url(base_url, &["authentication.do?value=(actions)=logout"])
        }
        Endpoints::Saldo => build_url(base_url, &["balanceinquiry.do"]),
        Endpoints::PubIp => build_url("http://icanhazip.com", &[""]),
    }
}

// Req is the reusable http client for the entire program.
// consists of the reusable http client itself, the uri (mutable), and the body (to be used for
// POST, PUT methods).
#[derive(Debug)]
pub struct Client {
    cstore: CookieJar,
}

impl Client {
    // Create default Client struct
    pub fn new() -> Result<Self> {
        let cstore = CookieJar::default();
        Ok(Client { cstore })
    }

    fn add_default_headers(&self, mut request: Request<Body>) -> Request<Body> {
        request.headers_mut()
            .insert(
        http::header::USER_AGENT,
        "Mozilla/5.0 (Linux; Android 9; 5032W) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.136 Mobile Safari/537.36".parse().unwrap(),
        );

        request.headers_mut().insert(
            http::header::UPGRADE_INSECURE_REQUESTS,
            "1".parse().unwrap(),
        );

        println!("Adds default request headers: {:?}", request.headers());
        request
    }

    fn add_cookie_header(&self, mut request: Request<Body>) -> Request<Body> {
        if let Some(header) = self.cstore.get_cookies(request.uri()) {
            request
                .headers_mut()
                .insert(http::header::COOKIE, header.parse().unwrap());
        }
        println!("Added cookie header: {:?}", request.headers());
        request
    }

    fn save_response_to_cookie(&mut self, resp: Response<Body>) -> Response<Body> {
        if resp.headers().contains_key(SET_COOKIE) {
            let cookies = resp
                .headers()
                .get_all(SET_COOKIE)
                .into_iter()
                .filter_map(|h| {
                    h.to_str().ok().or_else(|| {
                        tracing::warn!("invalid Set-Cookie encoding");
                        None
                    })
                })
                .filter_map(|h| {
                    resp.effective_uri()
                        .and_then(|uri| Cookie::parse(h, uri))
                        .or_else(|| {
                            tracing::warn!("could not parse Set-Cookie header");
                            None
                        })
                });
            self.cstore.add(cookies)
        }
        resp
    }

    pub fn simple_get(&self, u: &Url) -> Result<String> {
        let mut req = Request::get(u.as_str()).body(Body::empty())?;
        req = self.add_default_headers(req);
        let mut resp = req.send()?;
        Ok(resp.text()?)
    }

    pub fn get(&mut self, u: &Url) -> Result<String> {
        let mut req = Request::get(u.as_str())
            .redirect_policy(isahc::config::RedirectPolicy::Limit(10))
            .auto_referer()
            .body(Body::empty())?;
        req = self.add_default_headers(req);
        req = self.add_cookie_header(req);
        let mut resp = req.send()?;
        println!("Response headers: {:?}", resp.headers());
        let text = resp.text()?;
        self.save_response_to_cookie(resp);
        Ok(text)
    }

    pub fn post<I, K, V>(&mut self, u: &Url, form: Option<I>) -> Result<String>
    where
        I: IntoIterator,
        I::Item: core::borrow::Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut req: Request<Body>;
        match form {
            Some(f) => {
                let mut form_data = form_urlencoded::Serializer::new(String::new());
                form_data.extend_pairs(f);
                let form_string: String = form_data.finish().into();
                req = Request::post(u.as_str())
                    .redirect_policy(isahc::config::RedirectPolicy::Limit(10))
                    .body(Body::from_bytes(form_string.into_bytes()))?;
            }
            None => {
                req = Request::post(u.as_str())
                    .redirect_policy(isahc::config::RedirectPolicy::Limit(10))
                    .body(Body::empty())?;
            }
        }
        req = self.add_default_headers(req);
        req = self.add_cookie_header(req);
        let mut resp = req.send()?;
        println!("Response header: {:?}", resp.headers());
        let text = resp.text()?;
        self.save_response_to_cookie(resp);
        Ok(text)
    }
}
