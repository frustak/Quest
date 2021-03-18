//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{App, InputMode, Quest};

/// Split terminal view
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    chunks
}

/// Input field to make a new quest
pub fn quest_input(app: &App) -> Paragraph {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("New Quest")
                .border_type(BorderType::Rounded),
        );

    input
}

/// Help text
pub fn navigation_hint(input_mode: &InputMode) -> Paragraph {
    let (msg, style) = match input_mode {
        InputMode::Normal => (
            vec![
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" exit | "),
                Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" new quest | "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" check/uncheck quest | "),
                Span::styled("↑ ↓", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" navigate list | "),
                Span::styled("Delete", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" delete quest"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" stop adding | "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" save quest"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

/// Widget to show a list of quests
pub fn quest_list(quests: &[Quest], selected_quest: Option<usize>) -> List {
    // Map quests to ListItem widget
    let quests: Vec<ListItem> = quests
        .iter()
        .enumerate()
        .map(|(i, q)| {
            if let Some(highlighted) = selected_quest {
                quest_item(&q.title, q.completed, highlighted == i)
            } else {
                quest_item(&q.title, q.completed, false)
            }
        })
        .collect();

    List::new(quests)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Quests")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}

/// Widget to show a single quest
pub fn quest_item(title: &str, completed: bool, highlighted: bool) -> ListItem {
    let quest = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", Style::default().fg(Color::Green)),
            Span::styled(title, Style::default().add_modifier(Modifier::CROSSED_OUT)),
        ]))
    } else {
        ListItem::new(Spans::from(vec![Span::raw("   "), Span::raw(title)]))
    };

    if highlighted {
        quest.style(Style::default().bg(Color::Yellow).fg(Color::Black))
    } else {
        quest
    }
}
