use anyhow::Error;
// use structopt::StructOpt;

// modules
mod client;

// #[derive(Debug, StructOpt)]
// struct ReqOpt {
//     uri: Uri,
// }

#[derive(Copy, Clone, Debug)]
struct AppState {
    is_logged_in: bool,
}

impl AppState {
    fn new() -> Self {
        AppState{
            // is logged in to klikbca individual
            is_logged_in: false,
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    // let req_opt = ReqOpt::from_args();
    let app_state = AppState::new();
    let mut new_client = client::Client::new()?;

    let pub_ip = new_client.get_pub_ip().await?;
    println!("Current Public IP: {}", pub_ip);

    Ok(())
}
