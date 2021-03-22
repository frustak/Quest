mod events;
mod file_handler;
mod widget;

use crossterm::{
    event::{read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::handle_events;
use file_handler::{load_quests, save_quests};
use quest_tui::{App, CrossTerminal, DynResult, InputMode, Quest, TerminalFrame};
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
    execute!(stdout, EnterAlternateScreen)?;
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

/// Handle cursor when typing
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
