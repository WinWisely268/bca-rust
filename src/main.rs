// externs

// modules
mod accounts;
mod cookies;
mod clients;
mod states;

// use
use accounts::BcaAccount;
use anyhow::Result;
use clients::Client;
use states::AppState;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "env")]
struct ReqOpt {
    #[structopt(short = "u", env = "BCA_ACCOUNT")]
    user: String,
    #[structopt(short = "p", env = "BCA_PASSWORD")]
    password: String,
}

fn main() -> Result<()> {
    let opt = ReqOpt::from_args();
    let mut app_state = AppState::new();
    let acc = BcaAccount::new(opt.user, opt.password);
    let mut new_client = Client::new()?;

    acc.login(&mut new_client, &mut app_state)?;
    acc.get_saldo(&mut new_client, &mut app_state)?;

    Ok(())
}
