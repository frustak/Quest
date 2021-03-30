//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{App, InputMode};

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

/// Widget to show a list of quests
pub fn quest_list(app: &App) -> List {
    // Map quests to ListItem widget
    let quests: Vec<ListItem> = app
        .quests
        .iter()
        .enumerate()
        .map(|(i, q)| {
            if let Some(highlighted) = app.selected_quest {
                quest_item(q.title.clone(), q.completed, highlighted == i, app)
            } else {
                quest_item(q.title.clone(), q.completed, false, app)
            }
        })
        .collect();

    List::new(quests).style(app.default_style()).block(
        Block::default()
            .title("Quests")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Widget to show a single quest
pub fn quest_item(title: String, completed: bool, highlighted: bool, app: &App) -> ListItem {
    let style = if highlighted {
        app.selection_style()
    } else {
        app.default_style()
    };

    let quest = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", app.check_sign_style(highlighted)),
            Span::styled(title, app.checked_quest_style(highlighted)),
        ]))
    } else {
        ListItem::new(Spans::from(vec![
            Span::styled("   ", style),
            Span::styled(title, style),
        ]))
    };

    quest.style(style)
}

/// Input field to make a new quest
pub fn quest_input(app: &App) -> Paragraph {
    let style = match app.input_mode {
        InputMode::Normal => app.default_style(),
        InputMode::Adding => app.default_style().fg(app.configs.colors.selection_bg),
    };

    let input = Paragraph::new(app.input.as_ref()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("New Quest")
            .border_type(BorderType::Rounded)
            .style(style),
    );

    input
}

/// Help text
pub fn navigation_hint(app: &App) -> Paragraph {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled("q", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" exit | ", app.default_style()),
                Span::styled("n", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" new quest | ", app.default_style()),
                Span::styled("Enter", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" check/uncheck quest | ", app.default_style()),
                Span::styled("↑ ↓", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" navigate list | ", app.default_style()),
                Span::styled("Delete", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" delete quest", app.default_style()),
            ],
            app.default_style().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Adding => (
            vec![
                Span::styled("Esc", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" stop adding | ", app.default_style()),
                Span::styled("Enter", app.default_style().add_modifier(Modifier::BOLD)),
                Span::styled(" save quest", app.default_style()),
            ],
            app.default_style(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text).style(app.default_style())
}
