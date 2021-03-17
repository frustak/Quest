use std::{error::Error, io::Stdout};

use tui::{backend::CrosstermBackend, Frame, Terminal};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Representation of a task
pub struct Quest {
    pub title: String,
    pub completed: bool,
}

impl Quest {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }
}

/// Input field possible states
pub enum InputMode {
    /// Not Editing
    Normal,
    Editing,
}

/// Application state
pub struct App {
    /// New quest input value
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// List of all quests
    pub quests: Vec<Quest>,
    /// Should be true when application wants to exit
    pub should_exit: bool,
    /// Current selected quest
    pub selected_quest: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            quests: Vec::new(),
            should_exit: false,
            selected_quest: None,
        }
    }
}
