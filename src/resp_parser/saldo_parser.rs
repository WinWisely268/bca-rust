use crate::resp_parser::resp_traits::{TuiList, TuiListCreator};
use anyhow::Result;
use scraper::{Html, Selector};
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct AccountBalance<'a> {
    account_number: Cow<'a, str>,
    account_currency: Cow<'a, str>,
    account_balance: Cow<'a, str>,
}

impl<'a> AccountBalance<'a> {
    pub fn new<S>(resp: S) -> Result<Self>
    where
        S: Into<Cow<'a, str>>,
    {
        let mut acc_bal = AccountBalance::default();
        let doc = Html::parse_document(&resp.into());
        let table_selector =
            Selector::parse("tr[bgcolor='#FFFFFF']>td").expect("saldo html document error");
        let mut rows = doc.select(&table_selector);
        acc_bal.account_number = rows
            .next()
            .expect("no saldo rows")
            .text()
            .collect::<String>()
            .into();
        acc_bal.account_currency = rows
            .next()
            .expect("no saldo rows")
            .text()
            .collect::<String>()
            .into();
        acc_bal.account_balance = rows
            .next()
            .expect("no saldo rows")
            .text()
            .collect::<String>()
            .into();
        Ok(acc_bal)
    }

    pub fn account_balance_list(&self) -> TuiList {
        self.to_tui_list()
    }
}

impl<'a> TuiListCreator for AccountBalance<'a> {
    fn to_tui_list(&self) -> TuiList {
        TuiList::with_items(vec![
            format!("{}", self.account_number).into(),
            format!("{} {}", self.account_currency, self.account_balance).into(),
        ])
    }
}
