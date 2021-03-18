mod file_handler;
mod widget;

use crossterm::{
    event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_handler::{load_quests, save_quests};
use quest::{App, CrossTerminal, DynResult, InputMode, Quest, TerminalFrame};
use std::{error::Error, io::stdout};
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

fn main() -> DynResult {
    let mut terminal = initialize_terminal()?;
    draw_ui(&mut terminal)?;
    cleanup_terminal(terminal)?;

    Ok(())
}

/// Setup terminal
fn initialize_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

/// Draw user interface to terminal
fn draw_ui(terminal: &mut CrossTerminal) -> DynResult {
    let quests = load_quests()?;
    let mut app = App::new(&quests);

    while !app.should_exit {
        terminal.draw(|f| {
            app_view(f, &app);
        })?;

        if let Event::Key(event) = read()? {
            handle_events(event, &mut app);
        }
    }

    save_quests(&app.quests)?;
    Ok(())
}

/// Return terminal to it's normal state
fn cleanup_terminal(mut terminal: CrossTerminal) -> DynResult {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

/// Application view
fn app_view(frame: &mut TerminalFrame, app: &App) {
    let main_chunks = widget::main_chunks(frame.size());

    let quest_list = widget::quest_list(&app.quests, app.selected_quest);
    frame.render_widget(quest_list, main_chunks[0]);

    let quest_input = widget::quest_input(&app);
    frame.render_widget(quest_input, main_chunks[1]);
    handle_input_cursor(&app, frame, &main_chunks);

    let navigation_hint = widget::navigation_hint(&app.input_mode);
    frame.render_widget(navigation_hint, main_chunks[2]);
}

/// Set cursor when typing
fn handle_input_cursor(app: &App, frame: &mut TerminalFrame, chunks: &[Rect]) {
    match app.input_mode {
        InputMode::Normal =>
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        {
            frame.set_cursor(frame.size().x, frame.size().y)
        }
        InputMode::Editing => {
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

/// Input events handler
fn handle_events(event: KeyEvent, app: &mut App) {
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
