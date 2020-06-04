// externs

// modules
mod accounts;
mod clients;
mod states;
mod resp_parser;
mod ui;
mod events;

// use
use accounts::BcaAccount;
use anyhow::Result;
use clients::Client;
use crate::events::event::{Config, Event, Events};
use states::states::AppState;
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
    acc.get_saldo(&mut new_client, &mut app_state)?;
    acc.get_mutasi(&mut new_client, &mut app_state)?;

    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| ui::draw(&mut f, &mut app_state))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Up => {
                    app_state.on_up();
                }
                Key::Down => {
                    app_state.on_down();
                }
                Key::Left => {
                    app_state.on_left();
                }
                Key::Right => {
                    app_state.on_right();
                }
                _ => {}
            },
            Event::Tick => {
                acc.get_saldo(&mut new_client, &mut app_state)?;
                acc.get_mutasi(&mut new_client, &mut app_state)?;
            }
        }
    }

    Ok(())
}
