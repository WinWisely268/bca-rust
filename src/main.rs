// externs

// modules
mod accounts;
mod clients;
mod states;
mod resp_parser;
mod events;

// use
use accounts::BcaAccount;
use anyhow::Result;
use clients::Client;
use crate::events::event::{Config, Event, Events};
use states::AppState;
use std::time::Duration;
use structopt::StructOpt;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tracing::Level;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Terminal,
};


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
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let mut app_state = AppState::new();
    let acc = BcaAccount::new(opt.user, opt.password);
    let mut new_client = Client::new()?;
    acc.login(&mut new_client, &mut app_state)?;
    let events = Events::with_config(Config{
        tick_rate: Duration::from_millis(5000),
        ..Config::default()
    });
    let saldo = acc.get_saldo(&mut new_client, &mut app_state)?;
    let mutasi = acc.get_mutasi(&mut new_client, &mut app_state)?;

    Ok(())
}
