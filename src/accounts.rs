use crate::clients::{endpoint_url as epu, Client, Endpoints};
use crate::resp_parser::{mutasi_parser::AccountMutasi, saldo_parser::AccountBalance};
use crate::states::states::AppState;
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
        state.toggle_login();
        Ok(())
    }

    fn check_login_status(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        if !state.is_logged_in {
            self.login(client, state)?;
        }
        state.toggle_login();
        Ok(())
    }

    fn to_menu_page(&self, client: &mut Client) -> Result<()> {
        let main_menu_url = epu(Endpoints::AccountStatement)?;
        let params = vec![("value(actions)", "menu")];
        client.post(&main_menu_url, Some(params))?;
        Ok(())
    }

    pub fn get_saldo(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        self.check_login_status(client, state)?;
        self.to_menu_page(client)?;
        let saldo_url = epu(Endpoints::BalanceInquiry)?;
        let resp = client.post(&saldo_url, None::<Vec<(&str, &str)>>)?;
        let saldo = AccountBalance::new(resp)?;

        state.update_balance(saldo);

        Ok(())
    }

    pub fn get_mutasi(&self, client: &mut Client, state: &mut AppState) -> Result<()> {
        self.check_login_status(client, state)?;
        self.to_menu_page(client)?;
        let main_menu_url = epu(Endpoints::AccountStatement)?;
        let mut params = vec![("value(actions)", "acct_stmt")];
        client.post(&main_menu_url, Some(params))?;

        let start_dt = state.start_date.format("%d").to_string();
        let start_mt= state.start_date.format("%m").to_string();
        let start_yr =  state.start_date.format("%Y").to_string();
        let end_dt = state.end_date.format("%d").to_string();
        let end_mt = state.end_date.format("%m").to_string();
        let end_yr = state.end_date.format("%Y").to_string();

        params = vec![
            ("r1", "1"),
            ("value(D1)", "0"),
            ("value(startDt)", &start_dt),
            ("value(startMt)", &start_mt),
            ("value(startYr)", &start_yr),
            ("value(endDt)", &end_dt),
            ("value(endMt)", &end_mt),
            ("value(endYr)", &end_yr),
            ("value(actions)", "acctstmtview"),
        ];

        let resp = client.post(&main_menu_url, Some(params))?;
        let acc_mut = AccountMutasi::new(resp)?;
        state.update_mutations(acc_mut);

        Ok(())
    }

    pub fn logout(&self, client: &mut Client, state: &mut AppState) -> Result<String> {
        let logout_url = epu(Endpoints::Authentication)?;
        let resp = client.get(&logout_url)?;
        state.is_logged_in = false;
        Ok(resp)
    }
}
