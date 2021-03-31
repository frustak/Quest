use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use tui::style::Color;

/// Application configs
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Configs {
    pub colors: Colors,
    pub keybindings: KeyBindings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    pub foreground: Color,
    pub background: Color,
    pub selection_fg: Color,
    pub selection_bg: Color,
    pub check_sign: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            selection_fg: Color::Black,
            selection_bg: Color::Yellow,
            check_sign: Color::Green,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    pub exit_app: KeyCode,
    pub new_quest: KeyCode,
    pub check_and_uncheck_quest: KeyCode,
    pub list_up: KeyCode,
    pub list_down: KeyCode,
    pub delete_quest: KeyCode,
    pub exit_adding: KeyCode,
    pub save_quest: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            exit_app: KeyCode::Char('q'),
            new_quest: KeyCode::Char('n'),
            check_and_uncheck_quest: KeyCode::Enter,
            list_up: KeyCode::Up,
            list_down: KeyCode::Down,
            delete_quest: KeyCode::Delete,
            exit_adding: KeyCode::Esc,
            save_quest: KeyCode::Enter,
        }
    }
}

/// Converts a `KeyCode` to `String`
pub fn keycode_to_string(keycode: KeyCode) -> String {
    let temp;

    let stringified = match keycode {
        KeyCode::Backspace => "Backspace",
        KeyCode::Enter => "Enter",
        KeyCode::Left => "←",
        KeyCode::Right => "→",
        KeyCode::Up => "↑",
        KeyCode::Down => "↓",
        KeyCode::Home => "Home",
        KeyCode::End => "End",
        KeyCode::PageUp => "Page Up",
        KeyCode::PageDown => "Page Down",
        KeyCode::Tab => "Tab",
        KeyCode::BackTab => "Back Tab",
        KeyCode::Delete => "Delete",
        KeyCode::Insert => "Insert",
        KeyCode::F(n) => {
            temp = format!("F{}", n);
            temp.as_str()
        }
        KeyCode::Char(char) => {
            temp = char.to_string();
            temp.as_str()
        }
        KeyCode::Null => "Null",
        KeyCode::Esc => "Esc",
    }
    .to_string();

    stringified
}
