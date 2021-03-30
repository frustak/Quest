use crate::{actions, App, InputMode};
use crossterm::event::{KeyCode, KeyEvent};

/// Input events handler
pub fn handle_events(event: KeyEvent, app: &mut App) {
    match app.input_mode {
        InputMode::Normal => handle_normal_events(app, event.code),
        InputMode::Adding => handle_adding_events(app, event.code),
    }
}

/// When user is viewing quests
fn handle_normal_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.configs.keybindings;

    if keycode == keybindings.new_quest {
        actions::new_quest(app);
    } else if keycode == keybindings.exit_app {
        actions::exit_app(app);
    } else if keycode == keybindings.list_up {
        actions::list_up(app);
    } else if keycode == keybindings.list_down {
        actions::list_down(app);
    } else if keycode == keybindings.check_and_uncheck_quest {
        actions::check_and_uncheck_quest(app);
    } else if keycode == keybindings.delete_quest {
        actions::delete_quest(app);
    }
}

/// When user adding a new quest
fn handle_adding_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.configs.keybindings;

    if keycode == keybindings.save_quest && !app.input.trim().is_empty() {
        actions::save_quest(app);
    } else if keycode == keybindings.exit_adding {
        actions::exit_adding(app);
    } else if let KeyCode::Char(c) = keycode {
        actions::input_add_char(app, c);
    } else if let KeyCode::Backspace = keycode {
        actions::input_del_char(app);
    }
}
