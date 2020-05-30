use anyhow::Result;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, instrument};
use std::borrow::Cow;

fn get_last_text_el(s: scraper::ElementRef, separator: &str) -> String {
    s.text().collect::<String>().trim()
        .to_string()
        .split(separator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .last()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
pub struct AccountMutasi<'am> {
    info: AccountInfo<'am>,
    tx: AccountTxes<'am>,
    summary: MutationSummary<'am>,
}

impl <'am>AccountMutasi<'am> {
    pub fn new<S>(resp: S) -> Result<Self> 
    where S: Into<Cow<'am, str>>{
        let doc: Html = Html::parse_document(&resp.into());
        let info: AccountInfo<'am> = AccountInfo::from_resp(&doc);
        let tx: AccountTxes<'am> = AccountTxes::from_resp(&doc)?;
        let summary: MutationSummary<'am> = MutationSummary::from_resp(&doc)?;

        Ok(AccountMutasi{
            info, tx, summary,
        })
    }
}

#[derive(Debug, Default)]
struct AccountInfo<'a> {
    account_number: Cow<'a, str>,
    owner_name: Cow<'a, str>,
    period: Cow<'a, str>,
    currency: Cow<'a, str>,
}

impl <'a>AccountInfo<'a> {
    #[instrument]
    pub fn from_resp(doc: &Html) -> Self {
        let mut acc_info = AccountInfo::default();
        let account_info_selector = Selector::parse(
            r#"table[border="0"][width="100%"][cellpadding="0"][cellspacing="0"][class="blue"]"#,
            )
            .expect("html document error");
        for row in doc.select(&account_info_selector) {
            let acc_num_sel = Selector::parse("tr:nth-child(2)").expect("table has no account number");
            for acc_num in row.select(&acc_num_sel) {
                let n = get_last_text_el(acc_num, ".");
                debug!("Account Number: {}", n);
                acc_info.account_number = n[1..].to_string().into();
            }
            let acc_owner_sel = Selector::parse("tr:nth-child(3)").expect("table has no account owner");
            for acc_owner in row.select(&acc_owner_sel) {
                let o = get_last_text_el(acc_owner, ":");
                debug!("Account Owner: {}", o);
                acc_info.owner_name = o.into();
            }
            let acc_period_sel =
                Selector::parse("tr:nth-child(4)").expect("table has no tx period specified");
            for acc_per in row.select(&acc_period_sel) {
                let p = get_last_text_el(acc_per, ":");
                debug!("Account Info Period: {}", p);
                acc_info.period = p.into();
            }
            let acc_cur_sel =
                Selector::parse("tr:nth-child(5)").expect("table has no account currency specified");
            for acc_cur in row.select(&acc_cur_sel) {
                let p = get_last_text_el(acc_cur, ":");
                debug!("Account Currency: {}", p);
                acc_info.currency = p.into();
            }
        }
        acc_info
    }
}

#[derive(Debug, Default)]
struct AccountTxes<'a> {
    txes: Option<Vec<AccountTx<'a>>>,
}

impl<'a> AccountTxes<'a> {
    pub fn from_resp(doc: &Html) -> Result<Self> {
        let mut txes_vec: Vec<AccountTx> = vec![];
        let acc_tx_selector = Selector::parse(
            r#"table[width="100%"][class="blue"]"#,
            ).expect("html document error");

        let acc_table_elements= doc.select(&acc_tx_selector).last().expect("no transactions found").html();
        // Bad practice: replacing <br> element
        let re_br = Regex::new(r"<[/]?br>").unwrap();
        let acc_table_string = re_br.replace_all(&acc_table_elements, " ");
        let tr_selector = Selector::parse(r#"tr[bgcolor]"#).expect("transaction fragment error");
        let acc_table_fragments = Html::parse_fragment(&acc_table_string);
        for tx in acc_table_fragments.select(&tr_selector) {
            let new_acc_tx = AccountTx::from_resp(&tx)?;
            txes_vec.push(new_acc_tx);
        }

        if txes_vec.is_empty() { () }

        Ok(AccountTxes{
            txes: Some(txes_vec)
        })
    }
}


#[derive(Debug, Default)]
struct AccountTx<'a> {
    // transaction date & notes
    tx_date: Cow<'a, str>,
    tx_note: Cow<'a, str>,
    tx_amount: Cow<'a, str>,
    tx_category: Cow<'a, str>,
}

impl<'a> AccountTx<'a> {
    pub fn from_resp(row: &ElementRef) -> Result<Self> {
        let mut acc_tx = AccountTx::default();
        let date_selector = Selector::parse(r#"tr>td[valign="top"]:first-of-type"#).expect("tx date not found");
        for td in row.select(&date_selector) {
            let d = td.text().collect::<Cow<'a, str>>();
            acc_tx.tx_date = d;
        }
        let note_amt_selector = Selector::parse("tr>td:first-of-type + td").expect("tx note and amount is invalid");
        for namt_sel in row.select(&note_amt_selector) {
            let note_amt = namt_sel.text().collect::<String>();
            let re_note = Regex::new(r"[\s]{1}+").unwrap();
            let note = note_amt
                .split(" ")
                .map(|s| re_note.replace_all(s, " ").into())
                .collect::<Vec<String>>()
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join(" ");
            acc_tx.tx_note = note.into();
            let tx_amt = note_amt
                .split(" ")
                .map(|s| s.to_string()).collect::<Vec<String>>().into_iter().last().unwrap();
            acc_tx.tx_amount = tx_amt.into();

        }
        let category_selector = Selector::parse(r#"tr>td[valign="top"]:last-of-type"#).expect("tx category not found");
        for cat_el in row.select(&category_selector) {
            let cat = cat_el.text().collect::<Cow<'a, str>>();
            acc_tx.tx_category = cat;
        }
        Ok(acc_tx)
    }
}

#[derive(Debug, Default)]
struct MutationSummary<'a> {
    balance_begin: Cow<'a, str>,
    total_credits: Cow<'a, str>,
    total_debits: Cow<'a, str>,
    balance_end: Cow<'a, str>,
}

impl <'a> MutationSummary<'a> {
    pub fn from_resp(doc: &Html) -> Result<Self>{
        let mut mut_sum = MutationSummary::default();
        let mut_sum_selector = Selector::parse(r#"table[width="97%"][cellspacing="0"][class="blue"]"#)
            .expect("html document error");
        for tr in doc.select(&mut_sum_selector) {
            let sum_selector = Selector::parse(r#"tr>td[align="left"]:nth-of-type(n+2)"#).expect("table mutation summary is not found");
            let mut rows = tr.select(&sum_selector);
            mut_sum.balance_begin = rows.next().unwrap().text().collect::<String>().into();
            mut_sum.total_credits = rows.next().unwrap().text().collect::<String>().into();
            mut_sum.total_debits = rows.next().unwrap().text().collect::<String>().into();
            mut_sum.balance_end = rows.next().unwrap().text().collect::<String>().into();
        }
        Ok(mut_sum)
    }
}

