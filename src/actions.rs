use crate::{App, InputMode, Quest};

pub fn new_quest(app: &mut App) {
    app.input_mode = InputMode::Adding;
    app.selected_quest = None;
}

pub fn exit_app(app: &mut App) {
    app.should_exit = true;
}

pub fn list_up(app: &mut App) {
    if let Some(index) = app.selected_quest {
        if index > 0 {
            app.selected_quest = Some(index - 1);
        }
    }
}

pub fn list_down(app: &mut App) {
    if let Some(index) = app.selected_quest {
        if index < app.quests.len() - 1 {
            app.selected_quest = Some(index + 1);
        }
    }
}

pub fn check_and_uncheck_quest(app: &mut App) {
    if let Some(index) = app.selected_quest {
        app.quests[index].completed = !app.quests[index].completed;
    }
}

pub fn delete_quest(app: &mut App) {
    if let Some(index) = app.selected_quest {
        app.quests.remove(index);
        if app.quests.is_empty() {
            app.selected_quest = None;
        } else if app.selected_quest.unwrap() == app.quests.len() {
            app.selected_quest = Some(app.quests.len() - 1);
        }
    }
}

pub fn save_quest(app: &mut App) {
    let new_quest = Quest::new(app.input.drain(..).collect());
    app.quests.push(new_quest);
}

pub fn exit_adding(app: &mut App) {
    app.input_mode = InputMode::Normal;
    app.selected_quest = Some(0);
}

pub fn input_add_char(app: &mut App, c: char) {
    app.input.push(c);
}

pub fn input_del_char(app: &mut App) {
    app.input.pop();
}
