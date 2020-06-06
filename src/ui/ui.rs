use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect, Direction},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, Row, Table, Text, Paragraph,
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
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(60),
            Constraint::Percentage(15),
        ].as_ref(),
        ).split(area);
    draw_input(f, app, chunks[0]);
    draw_balance(f, app, chunks[1]);
    draw_statements(f, app, chunks[2]);
    draw_summary(f, app, chunks[3]);
}

fn draw_input<B:Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100),
        ].as_ref())
        .margin(2)
        .split(area);
    
    let block = Block::default().borders(Borders::ALL).title("Input Start and End Date (dd/mm/yyyy)");
    f.render_widget(block, area);
    let text = [Text::raw(&app.input_string)];
    let input = Paragraph::new(text.iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("dd/mm/yyyy - dd/mm/yyyy"));
    f.render_widget(input, chunks[0]);
}

fn draw_balance<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .margin(2)
        .split(area);
    let block = Block::default().borders(Borders::BOTTOM).title("Account Balance");
    f.render_widget(block, area);
    let info = app.account_balance.items.iter().step_by(2)
        .map(|i| Text::raw(i))
        .collect::<Vec<Text>>();
    let info = Paragraph::new(info.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left)
        .wrap(true);
    f.render_widget(info, chunks[0]);

    let balance = app.account_balance.items.iter().skip(1)
            .map(|i| Text::styled(i, Style::default().fg(Color::Green)))
            .collect::<Vec<Text>>();
    let balance = Paragraph::new(balance.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right)
        .wrap(true);
    f.render_widget(balance, chunks[1]);
}

fn draw_statements<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Account Statements");
    f.render_widget(block, area);
    let statements = app.account_info.items
        .iter()
        .map(|i| {
         Text::raw(format!("{}\n", i))   
        })
        .collect::<Vec<Text>>();
    let statements = Paragraph::new(statements.iter())
        .block(Block::default().borders(Borders::BOTTOM))
        .alignment(Alignment::Center);
    f.render_widget(statements, chunks[0]);

    let tbl_selected_style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
    let tbl_normal_style = Style::default();
    let tbl_header = ["Date", "Note", "Amount", "Category"];
    let rows = app.account_mutations.items.iter()
        .map(|i| Row::StyledData(i.iter(), tbl_normal_style));
    let t = Table::new(tbl_header.iter(), rows)
        .block(Block::default().borders(Borders::ALL).title("Statements Table"))
        .highlight_style(tbl_selected_style)
        .highlight_symbol("➜ ")
        .widths(&[
                Constraint::Percentage(5),
                Constraint::Percentage(65),
                Constraint::Percentage(25),
                Constraint::Percentage(5),
        ]);
    f.render_stateful_widget(t, chunks[1], &mut app.account_mutations.state);
}

fn draw_summary<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .margin(2)
        .split(area);
    let block = Block::default().borders(Borders::BOTTOM).title("Account Summary");
    f.render_widget(block, area);
    let info = app.account_summary.items.iter().step_by(2)
        .map(|i| Text::raw(format!("{}\n", i)))
        .collect::<Vec<Text>>();
    let info = Paragraph::new(info.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left)
        .wrap(true);
    f.render_widget(info, chunks[0]);

    let summary = app.account_summary.items.iter().skip(1).step_by(2)
            .map(|i| Text::styled(format!("{}\n", i), Style::default().fg(Color::Yellow)))
            .collect::<Vec<Text>>();
    let summary = Paragraph::new(summary.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Right)
        .wrap(true);
    f.render_widget(summary, chunks[1]);

}
