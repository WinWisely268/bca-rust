use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Block, Borders, List, Paragraph, Row, Table, Text,
    },
    Frame,
};
use crate::states::states::AppState;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_account_statements(f, app, chunks[0]);
}

fn draw_account_statements<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(7),
        ].as_ref(),
        ).split(area);
    draw_balance(f, app, chunks[0]);
    draw_statements(f, app, chunks[1]);
    draw_summary(f, app, chunks[2]);
}

fn draw_balance<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(2)].as_ref())
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::BOTTOM).title("Account Balance");
    f.render_widget(block, area);
    let balance = app.account_balance.items.iter().map(|i| Text::raw(i));
    let balance = List::new(balance)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
        .highlight_symbol("➜ ");
    f.render_stateful_widget(balance, chunks[0], &mut app.account_balance.state);
}

fn draw_statements<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(2), Constraint::Min(5)].as_ref())
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Account Statements");
    f.render_widget(block, area);
    let statements = app.account_info.items.iter().map(|i| Text::raw(i));
    let statements = List::new(statements)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
        .highlight_symbol("➜ ");
    f.render_stateful_widget(statements, chunks[0], &mut app.account_info.state);

    let tbl_selected_style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
    let tbl_normal_style = Style::default().fg(Color::White);
    let tbl_header = ["Date", "Note", "Amount", "Category"]; 
    let rows = app.account_mutations.items.iter().map(|i| Row::StyledData(i.iter(), tbl_normal_style));
    let t = Table::new(tbl_header.iter(), rows)
        .block(Block::default().borders(Borders::ALL).title("Statements Table"))
        .highlight_style(tbl_selected_style)
        .highlight_symbol("➜ ")
        .widths(&[
            Constraint::Percentage(15),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(5),
        ]);
    f.render_stateful_widget(t, chunks[1], &mut app.account_mutations.state);
}

fn draw_summary<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
     let chunks = Layout::default()
        .constraints([Constraint::Min(2)].as_ref())
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::BOTTOM).title("Summary");
    f.render_widget(block, area);
    let summary = app.account_summary.items.iter().map(|i| Text::raw(i));
    let summary = List::new(summary)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::LightGreen).modifier(Modifier::BOLD))
        .highlight_symbol("➜ ");
    f.render_stateful_widget(summary, chunks[0], &mut app.account_summary.state);
   
}
