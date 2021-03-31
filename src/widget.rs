//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{configs::keycode_to_string, App, InputMode, Quest};

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

/// Shows a list of quests
pub fn quest_list(app: &App) -> List {
    // Map quests to ListItem widget
    let quests: Vec<ListItem> = app
        .quests
        .iter()
        .enumerate()
        .map(|q| indexed_quest_item(app, q))
        .collect();

    List::new(quests).style(app.default_style()).block(
        Block::default()
            .title("Quests")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Check if a quest is selected then renders it properly
fn indexed_quest_item<'a>(app: &'a App, (index, quest): (usize, &Quest)) -> ListItem<'a> {
    if let Some(selected_index) = app.selected_quest {
        quest_item(
            quest.title.clone(),
            quest.completed,
            selected_index == index,
            app,
        )
    } else {
        quest_item(quest.title.clone(), quest.completed, false, app)
    }
}

/// Widget to show a single quest
fn quest_item(title: String, completed: bool, selected: bool, app: &App) -> ListItem {
    let style = if selected {
        app.selection_style()
    } else {
        app.default_style()
    };

    let quest = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", app.check_sign_style(selected)),
            Span::styled(title, app.checked_quest_style(selected)),
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
    let keybindings = &app.configs.keybindings;

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_app),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" exit | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.new_quest),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" new quest | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.check_and_uncheck_quest),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" check/uncheck quest | ", app.default_style()),
                Span::styled(
                    format!(
                        "{}/{}",
                        keycode_to_string(keybindings.list_up),
                        keycode_to_string(keybindings.list_down)
                    ),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" navigate list | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.delete_quest),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" delete quest", app.default_style()),
            ],
            app.default_style().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Adding => (
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_adding),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" stop adding | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.save_quest),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" save quest", app.default_style()),
            ],
            app.default_style(),
        ),
    };

    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    Paragraph::new(help_text).style(app.default_style())
}
