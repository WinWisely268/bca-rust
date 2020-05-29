use crate::clients::{endpoint_url as epu, Client, Endpoints};
use crate::states::AppState;
use anyhow::Result;

// BcaAccount contains username and password
#[derive(Copy, Clone, Debug)]
pub struct BcaAccount {
    user: &'static str,
    password: &'static str,
}

impl BcaAccount {
    pub fn new(user: String, password: String) -> Self {
        let user_static = Box::leak(user.into_boxed_str());
        let password_static = Box::leak(password.into_boxed_str());
        BcaAccount {
            user: user_static,
            password: password_static,
        }
    }

    pub fn get_pub_ip(&self, client: &mut Client) -> Result<String> {
        let pub_ip_url = epu(Endpoints::PubIp)?;
        let resp = client.simple_get(&pub_ip_url)?;
        let ip = resp
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        Ok(ip)
    }

    pub fn login(
        &self,
        client: &mut Client,
        state: &mut AppState,
    ) -> Result<String> {
        let ip = self.get_pub_ip(client)?;
        let params: Vec<(&str, &str)> = vec![
            ("value(user_id)", self.user),
            ("value(pswd)", self.password),
            ("value(Submit)", "LOGIN"),
            ("value(actions)", "login"),
            ("value(user_ip)", ip.as_ref()),
            ("user_ip", ip.as_ref()),
            ("value(mobile)", "true"),
            ("mobile", "true"),
        ];
        let login_url = epu(Endpoints::LoginAction)?;
        let resp = client.post(&login_url, Some(params))?;
        println!("{}", resp);
        state.toggle_login();
        Ok(resp)
    }

    pub fn get_saldo(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        if !state.is_logged_in {
            self.login(client, state)?;
        }
        let saldo_url = epu(Endpoints::Saldo)?;
        let resp = client.get(&saldo_url)?;
        println!("{}", resp);
        Ok(resp)
    }

    pub fn logout(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        let logout_url = epu(Endpoints::LogoutAction)?;
        let resp = client.post(&logout_url, None::<Vec<(&str, &str)>>)?;
        println!("{}", resp);
        state.toggle_login();
        Ok(resp)
    }
}
