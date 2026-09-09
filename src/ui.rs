use crate::app::{App, Focus};
use crate::session::Status;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // List
            Constraint::Length(3), // Hints
            Constraint::Length(3), // Input
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("★ Satra v0.1.0")
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(header, chunks[0]);

    // Session List
    let items: Vec<ListItem> = app
        .sessions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let status_color = match s.status {
                Status::Running => Color::Green,
                Status::Exited(_) => Color::Red,
            };
            let status_dot = Span::styled("●", Style::default().fg(status_color));
            let pid = s.pid;
            let uptime = s.uptime().as_secs();
            let mins = uptime / 60;
            let secs = uptime % 60;

            let exit_info = match s.status {
                Status::Exited(Some(code)) => format!(" [exit:{}]", code),
                Status::Exited(None) => " [killed]".to_string(),
                Status::Running => "".to_string(),
            };

            let content = format!(
                " [{}]  {}   PID:{}  ↑{}m{}s{}",
                s.id, s.command, pid, mins, secs, exit_info
            );
            let line = vec![status_dot, Span::raw(content)];

            if matches!(app.focus, Focus::List) && i == app.selected_index {
                ListItem::new(Line::from(line)).style(Style::default().bg(Color::DarkGray))
            } else {
                ListItem::new(Line::from(line))
            }
        })
        .collect();

    let list_border_color = if matches!(app.focus, Focus::List) {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(list_border_color))
            .title(" Sessions "),
    );

    f.render_widget(list, chunks[1]);

    // Hints
    let hints = Paragraph::new("Tab:⇆  ↑↓:nav  q:quit(detail)  Enter:action/detail")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(hints, chunks[2]);

    // Input
    let input_border_color = if matches!(app.focus, Focus::Input) {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let input_text = format!("> {}", app.input);
    let input = Paragraph::new(input_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(input_border_color)),
    );
    f.render_widget(input, chunks[3]);

    // Show cursor in input when focused
    if matches!(app.focus, Focus::Input) {
        f.set_cursor(
            chunks[3].x + app.input.len() as u16 + 3, // +3 for border + "> "
            chunks[3].y + 1,
        );
    }

    if app.show_detail {
        draw_detail_popup(f, app);
    }
}

fn draw_detail_popup(f: &mut Frame, app: &App) {
    if let Some(session) = app.sessions.get(app.selected_index) {
        let area = centered_rect(60, 40, f.size());
        f.render_widget(Clear, area);

        let status_str = match session.status {
            Status::Running => "Running".to_string(),
            Status::Exited(Some(c)) => format!("Exited ({})", c),
            Status::Exited(None) => "Terminated".to_string(),
        };

        let text = vec![
            Line::from(vec![
                Span::styled("Command: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&session.command),
            ]),
            Line::from(vec![
                Span::styled("PID: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(session.pid.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(status_str),
            ]),
            Line::from(vec![
                Span::styled("Uptime: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!(
                    "{}m {}s",
                    session.uptime().as_secs() / 60,
                    session.uptime().as_secs() % 60
                )),
            ]),
            Line::from(""),
            Line::from("Actions:"),
            Line::from("  [k] Kill process"),
            Line::from("  [r] Restart process"),
            Line::from("  [d] Delete from list"),
            Line::from("  [q] Close panel"),
        ];

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Session Detail ")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
