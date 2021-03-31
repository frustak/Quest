use crate::{actions, App, InputMode, TerminalFrame};
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::Rect;

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

/// Handle cursor when typing
pub fn handle_input_cursor(app: &App, frame: &mut TerminalFrame, chunks: &[Rect]) {
    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => (),
        InputMode::Adding => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.len() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
}
