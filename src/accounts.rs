use crate::clients::{endpoint_url as epu, Client, Endpoints};
use crate::resp_parser::saldo_parser::parse_saldo;
use crate::resp_parser::mutasi_parser::AccountMutasi;
use crate::states::AppState;
use anyhow::Result;
use tracing::{debug, info, instrument};

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

    fn get_pub_ip(&self, client: &mut Client) -> Result<String> {
        let pub_ip_url = epu(Endpoints::PubIp)?;
        let resp = client.simple_get(&pub_ip_url)?;
        let ip = resp
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        Ok(ip)
    }

    pub fn login(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        let ip = self.get_pub_ip(client)?;
        let loginform_url = epu(Endpoints::Login)?;
        client.get(&loginform_url)?;
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
        let login_url = epu(Endpoints::Authentication)?;
        let resp = client.post(&login_url, Some(params))?;
        state.toggle_login();
        Ok(resp)
    }

    #[instrument]
    fn check_login_status(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        if !state.is_logged_in {
            debug!("not logged in, login first");
            self.login(client, state)?;
        }
        Ok(())
    }

    fn to_menu_page(&self, client: &mut Client) -> Result<()> {
        let main_menu_url = epu(Endpoints::AccountStatement)?;
        let params = vec![("value(actions)", "menu")];
        client.post(&main_menu_url, Some(params))?;
        Ok(())
    }

    #[instrument]
    pub fn get_saldo(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        self.check_login_status(client, state)?;
        self.to_menu_page(client)?;
        let saldo_url = epu(Endpoints::BalanceInquiry)?;
        let resp = client.post(&saldo_url, None::<Vec<(&str, &str)>>)?;
        let saldo = parse_saldo(resp.as_str())?;
        info!("Current Balance / Saldo: {:?}", saldo);
        Ok(resp)
    }

    #[instrument]
    pub fn get_mutasi(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        self.check_login_status(client, state)?;
        self.to_menu_page(client)?;
        let main_menu_url = epu(Endpoints::AccountStatement)?;
        let mut params = vec![("value(actions)", "acct_stmt")];
        client.post(&main_menu_url, Some(params))?;

        params = vec![
            ("r1", "1"),
            ("value(D1)", "0"),
            ("value(startDt)", "23"),
            ("value(startMt)", "05"),
            ("value(startYr)", "2020"),
            ("value(endDt)", "30"),
            ("value(endMt)", "05"),
            ("value(endYr)", "2020"),
            ("value(actions)", "acctstmtview"),
        ];

        let resp = client.post(&main_menu_url, Some(params))?;
        let acc_mut = AccountMutasi::new(resp)?;

        info!("Account Mutasi: {:?}", acc_mut);

        Ok("".to_string())
    }

    #[instrument]
    pub fn logout(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        let logout_url = epu(Endpoints::Authentication)?;
        let resp = client.get(&logout_url)?;
        debug!("Response triggered: {}", resp);
        state.toggle_login();
        Ok(resp)
    }
}
