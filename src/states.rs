use crate::resp_parser::{
    saldo_parser::AccountBalance,
    mutasi_parser::AccountMutasi,
    resp_traits::{TuiList, TuiTable},
};
// Global AppState
#[derive(Clone)]
pub struct AppState {
    pub is_logged_in: bool,
    pub account_info: TuiList,
    pub account_mutations: TuiTable,
    pub account_balance: TuiList,
    pub account_summary: TuiList,
}

impl AppState {
    pub fn new() -> Self {
        AppState{
            // is logged in to klikbca individual
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
}


