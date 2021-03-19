use crossterm::event::{KeyCode, KeyEvent};
use quest_tui::{App, InputMode, Quest};

/// Input events handler
pub fn handle_events(event: KeyEvent, app: &mut App) {
    match app.input_mode {
        InputMode::Normal => handle_normal_events(app, event.code),
        InputMode::Editing => handle_editing_events(app, event.code),
    }
}

/// When user is viewing quests
fn handle_normal_events(app: &mut App, keycode: KeyCode) {
    match keycode {
        KeyCode::Char('n') => {
            app.input_mode = InputMode::Editing;
            app.selected_quest = None;
        }
        KeyCode::Char('q') => {
            app.should_exit = true;
        }
        KeyCode::Up => {
            if let Some(index) = app.selected_quest {
                if index > 0 {
                    app.selected_quest = Some(index - 1);
                }
            }
        }
        KeyCode::Down => {
            if let Some(index) = app.selected_quest {
                if index < app.quests.len() - 1 {
                    app.selected_quest = Some(index + 1);
                }
            }
        }
        KeyCode::Enter => {
            if let Some(index) = app.selected_quest {
                app.quests[index].completed = !app.quests[index].completed;
            }
        }
        KeyCode::Delete => {
            if let Some(index) = app.selected_quest {
                app.quests.remove(index);
                if app.quests.is_empty() {
                    app.selected_quest = None;
                } else if app.selected_quest.unwrap() == app.quests.len() {
                    app.selected_quest = Some(app.quests.len() - 1);
                }
            }
        }
        _ => {}
    }
}

/// When user adding new quest
fn handle_editing_events(app: &mut App, keycode: KeyCode) {
    match keycode {
        KeyCode::Enter if !app.input.trim().is_empty() => {
            let new_quest = Quest::new(app.input.drain(..).collect());
            app.quests.push(new_quest);
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.selected_quest = Some(0);
        }
        _ => {}
    }
}
