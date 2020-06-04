use crate::clients::{endpoint_url as epu, Client, Endpoints};
use crate::resp_parser::{mutasi_parser::AccountMutasi, saldo_parser::AccountBalance};
use crate::states::states::AppState;
use anyhow::Result;
use tracing::{event, Level};

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

    pub fn login(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
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
        client.post(&login_url, Some(params))?;
        state.is_logged_in = true;
        Ok(())
    }

    fn check_login_status(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        let span = tracing::span!(Level::DEBUG, "Check Login Status");
        if !state.is_logged_in {
            event!(parent: &span, Level::DEBUG, "not logged in, login first");
            self.login(client, state)?;
        }
        state.is_logged_in = true;
        Ok(())
    }

    fn to_menu_page(&self, client: &mut Client) -> Result<()> {
        let main_menu_url = epu(Endpoints::AccountStatement)?;
        let params = vec![("value(actions)", "menu")];
        client.post(&main_menu_url, Some(params))?;
        Ok(())
    }

    pub fn get_saldo(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        let span = tracing::span!(Level::DEBUG, "Check Current Balance");
        self.check_login_status(client, state)?;
        self.to_menu_page(client)?;
        let saldo_url = epu(Endpoints::BalanceInquiry)?;
        let resp = client.post(&saldo_url, None::<Vec<(&str, &str)>>)?;
        let saldo = AccountBalance::new(resp)?;
        event!(parent: &span, Level::DEBUG, "Current Balance / Saldo: {:?}", saldo);

        state.update_balance(saldo);

        Ok(())
    }

    pub fn get_mutasi(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        let span = tracing::span!(Level::DEBUG, "Check Mutasi");
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
        event!(parent: &span, Level::DEBUG, "Account Mutasi: {:?}", acc_mut);
        state.update_mutations(acc_mut);

        Ok(())
    }

    pub fn logout(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        let span = tracing::span!(Level::DEBUG, "Logout Operation");
        let logout_url = epu(Endpoints::Authentication)?;
        let resp = client.get(&logout_url)?;
        event!(parent: &span, Level::DEBUG, "Response triggered: {}", resp);
        state.is_logged_in = false;
        Ok(resp)
    }
}
