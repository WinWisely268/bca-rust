use anyhow::{Result, anyhow};
use crate::resp_parser::{
    saldo_parser::AccountBalance,
    mutasi_parser::AccountMutasi,
    resp_traits::{TuiList, TuiTable},
};
use chrono::{DateTime, NaiveDate, Duration, offset::Local};

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Editing,
}
// Global AppState
#[derive(Clone)]
pub struct AppState {
    pub is_logged_in: bool,
    pub input_string: String,
    pub input_mode: InputMode,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub account_info: TuiList,
    pub account_mutations: TuiTable,
    pub account_balance: TuiList,
    pub account_summary: TuiList,
}

impl AppState {
    pub fn new() -> Self {
        let now = Local::now();
        AppState{
            // is logged in to klikbca individual
            input_string: String::new(),
            input_mode: InputMode::Normal,
            start_date: add_date(now, Duration::days(-7)),
            end_date: add_date(now, Duration::seconds(0)),
            is_logged_in: false,
            account_info: TuiList::new(),
            account_mutations: TuiTable::new(Vec::new()),
            account_balance: TuiList::new(),
            account_summary: TuiList::new(),
        }
    }

    pub fn update_balance(&mut self, saldo: AccountBalance) {
        self.account_balance = saldo.account_balance_list();
    }

    pub fn update_mutations(&mut self, mutasi: AccountMutasi) {
        self.account_mutations = mutasi.account_tx_table();
        self.account_info = mutasi.account_info_list();
        self.account_summary = mutasi.account_summary_list();
    }

    pub fn update_dates(&mut self) -> Result<()>{
        let dates = self.input_string.split("-").into_iter()
            .map(|s| s.trim()).collect::<Vec<&str>>();
        if dates.len() != 2 {
            return Err(anyhow!("Error: no date range detected"));
        }
        if let Some(d) = dates.first() {
            let start_date = NaiveDate::parse_from_str(d, "%d/%m/%Y")?;
            self.start_date = start_date;
        }
        if let Some(d) = dates.last() {
            let end_date = NaiveDate::parse_from_str(d, "%d/%m/%Y")?;
            self.end_date = end_date;
        }
        Ok(())
    }

    pub fn toggle_login(&mut self) {
        self.is_logged_in = !self.is_logged_in;
    }

    pub fn on_up(&mut self) {
        self.account_mutations.previous();
    }

    pub fn on_down(&mut self) {
        self.account_mutations.next();
    }
}


fn add_date(cur: DateTime<Local>, dur: Duration) -> NaiveDate {
    let added_now = cur.checked_add_signed(dur);
    match added_now {
        None => cur.naive_local().date(),
        Some(d) => d.naive_local().date(),
    }
}
