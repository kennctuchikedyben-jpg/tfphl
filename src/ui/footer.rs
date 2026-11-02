use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::app_state::AppState;

pub fn render_footer(f: &mut Frame, state: &AppState, area: Rect) {
    let msg = if let Some(ref err) = state.error_message {
        Line::from(vec![
            Span::styled(" Error: ", Style::default().fg(Color::White).bg(Color::Red)),
            Span::raw(" "),
            Span::styled(err, Style::default().fg(Color::LightRed)),
        ])
    } else if state.loading {
        Line::from(vec![
            Span::styled(" Fetching FPHL data from HockeyShift... ", Style::default().fg(Color::Yellow)),
        ])
    } else {
        Line::from(vec![
            Span::styled("[1-3]", Style::default().fg(Color::Yellow)),
            Span::raw(" Tabs  "),
            Span::styled("[h/l, ◄/►]", Style::default().fg(Color::Yellow)),
            Span::raw(" Date  "),
            Span::styled("[t]", Style::default().fg(Color::Yellow)),
            Span::raw(" Today  "),
            Span::styled("[j/k, ▲/▼]", Style::default().fg(Color::Yellow)),
            Span::raw(" Select  "),
            Span::styled("[Enter]", Style::default().fg(Color::Yellow)),
            Span::raw(" Boxscore  "),
            Span::styled("[Tab]", Style::default().fg(Color::Yellow)),
            Span::raw(" Toggle  "),
            Span::styled("[f]", Style::default().fg(Color::Yellow)),
            Span::raw(" Fav Team  "),
            Span::styled("[r]", Style::default().fg(Color::Yellow)),
            Span::raw(" Refresh  "),
            Span::styled("[?]", Style::default().fg(Color::Yellow)),
            Span::raw(" Help  "),
            Span::styled("[q]", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ])
    };

    let p = Paragraph::new(msg).block(Block::default().borders(Borders::TOP));
    f.render_widget(p, area);
}
