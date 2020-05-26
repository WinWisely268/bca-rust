use crate::clients::{Endpoints, Req};
use crate::states::AppState;
use anyhow::Error;

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

    pub async fn login(
        &self,
        client: &mut Req,
        ip: String,
        state: &mut AppState,
    ) -> Result<String, Error> {
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
        client.set_url(Endpoints::LoginAction);
        let resp = client.post_form(params).await?;
        println!("{}", resp);
        state.toggle_login();
        Ok("".to_string())
    }
}
