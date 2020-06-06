// externs

// modules
mod accounts;
mod clients;
mod events;
mod resp_parser;
mod states;
mod ui;

// use
use crate::events::event::{Config, Event, Events};
use accounts::BcaAccount;
use anyhow::Result;
use clients::Client;
use states::states::{AppState, InputMode};
use std::io::{self, Write};
use std::time::Duration;
use structopt::StructOpt;
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};
use unicode_width::UnicodeWidthStr;

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
    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(2000),
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
        terminal.draw(|mut f| ui::ui::draw(&mut f, &mut app_state))?;

        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app_state.input_string.width() as u16, 5)
        )?;
        io::stdout().flush().ok();

        if let Event::Input(input) = events.next()? {
            match app_state.input_mode {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app_state.input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    }
                    Key::Char('q') => {
                        acc.logout(&mut new_client, &mut app_state)?;
                        break;
                    }
                    Key::Up => app_state.on_up(),
                    Key::Down => app_state.on_down(),
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        app_state.update_dates()?;
                    }
                    Key::Char(c) => {
                        app_state.input_string.push(c);
                    }
                    Key::Backspace => {
                        app_state.input_string.pop();
                    }
                    Key::Esc => {
                        app_state.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    _ => {}
                },
            }
        }
        if let Event::Tick = events.next()? {
            acc.get_saldo(&mut new_client, &mut app_state)?;
            acc.get_mutasi(&mut new_client, &mut app_state)?;
        }
    }
    Ok(())
}
