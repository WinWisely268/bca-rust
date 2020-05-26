use anyhow::Error;
use http::Uri;
use reqwest::{header, header::HeaderMap, Client, ClientBuilder};

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
        Endpoints::LoginAction => [base_url, "authentication.do"]
            .concat()
            .parse::<Uri>()
            .unwrap(),
        Endpoints::LogoutAction => [base_url, "authentication.do?value=(actions)=logout"]
            .concat()
            .parse::<Uri>()
            .unwrap(),
        Endpoints::Saldo => [base_url, "balanceinquiry.do"]
            .concat()
            .parse::<Uri>()
            .unwrap(),
        Endpoints::PubIp => "http://icanhazip.com".parse::<Uri>().unwrap(),
    }
}

// build http client with default headers and cookies support
fn build_client() -> Result<Client, Error> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        header::HOST,
        "m.klikbca.com".parse().expect("Invalid HOST value"),
    );
    default_headers.insert(
        header::CONNECTION,
        "keepalive".parse().expect("Invalid keepalive header"),
    );
    default_headers.insert(
        header::CACHE_CONTROL,
        "max-age=0".parse().expect("Invalid max-age"),
    );
    default_headers.insert(
        header::UPGRADE_INSECURE_REQUESTS,
        "1".parse().expect("Invalid insecure request upgrade"),
    );
    default_headers.insert(
        header::USER_AGENT,
        "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2490.76 Mobile Safari/537.36"
        .parse().expect("Invalid UA"),
    );
    default_headers.insert(
        header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            .parse()
            .expect("Invalid accept"),
    );
    default_headers.insert(
        header::ACCEPT_ENCODING,
        "gzip, deflate, sdch, br"
            .parse()
            .expect("Invalid accept encoding"),
    );
    default_headers.insert(
        header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.8,id;q=0.6,fr;q=0.4"
            .parse()
            .expect("Invalid accept language"),
    );

    let cb = ClientBuilder::new()
        .default_headers(default_headers)
        .cookie_store(true)
        .build()?;
    Ok(cb)
}

// Req is the reusable http client for the entire program.
// consists of the reusable http client itself, the uri (mutable), and the body (to be used for
// POST, PUT methods).
#[derive(Debug)]
pub struct Req {
    // hc is reqwest::Client
    hc: Client,
    // uri -> url for operation (this is mutable)
    uri: Uri,
}

impl Req {
    // Create default Client struct
    pub fn new() -> Result<Self, Error> {
        let client = build_client()?;
        Ok(Req {
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
        let ip = pub_ip_string
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        Ok(ip)
    }

    // Async GET
    pub async fn get(&self) -> Result<String, Error> {
        let url: String = format!("{}", self.uri);
        let get_url: &str = url.as_str();
        let resp = self.hc.get(get_url).send().await?;
        Ok(resp.text().await?)
    }

    // Async POST, takes body (can be a string literal)
    pub async fn post_form(&self, form: Vec<(&str, &str)>) -> Result<String, Error> {
        let url: String = format!("{}", self.uri);
        let post_url: &str = url.as_str();
        let resp = self.hc.post(post_url).form(&form).send().await?;
        Ok(resp.text().await?)
    }
}
