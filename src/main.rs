// externs

// modules
mod accounts;
mod clients;
mod states;

// use
use accounts::BcaAccount;
use anyhow::Error;
use clients::Req;
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = ReqOpt::from_args();
    let mut app_state = AppState::new();
    let acc = BcaAccount::new(opt.user, opt.password);
    let mut new_client = Req::new()?;

    let pub_ip = new_client.get_pub_ip().await?;
    println!("Current Public IP: {}", pub_ip);

    acc.login(&mut new_client, pub_ip, &mut app_state).await?;

    Ok(())
}
