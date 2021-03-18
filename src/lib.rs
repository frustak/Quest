use serde::{Deserialize, Serialize};
use std::{error::Error, io::Stdout};
use tui::{backend::CrosstermBackend, Frame, Terminal};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Represent a task
#[derive(Serialize, Deserialize, Clone)]
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

/// Represent a list of tasks
#[derive(Serialize, Deserialize, Default)]
pub struct QuestList {
    pub quests: Vec<Quest>,
}

impl QuestList {
    pub fn new(quests: &[Quest]) -> Self {
        Self {
            quests: quests.to_vec(),
        }
    }
}

/// Possible Input field states
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

impl App {
    pub fn new(quests: &[Quest]) -> Self {
        Self {
            quests: quests.to_vec(),
            selected_quest: Some(0),
            input: String::new(),
            input_mode: InputMode::Normal,
            should_exit: false,
        }
    }
}
