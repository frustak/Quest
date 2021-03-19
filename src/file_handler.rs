use lazy_static::lazy_static;
use quest_tui::{DynResult, Quest, QuestList};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref FILE_PATH: PathBuf = dirs::data_dir().unwrap().join("quests.toml");
}

/// Save all quests to a file
pub fn save_quests(quests: &[Quest]) -> DynResult {
    let quests = &QuestList::new(quests);
    let stringified_quests = toml::to_string(quests)?;
    fs::write(FILE_PATH.as_path(), stringified_quests)?;

    Ok(())
}

/// Load all saved quests from file
pub fn load_quests() -> Result<Vec<Quest>, io::Error> {
    if !Path::new(FILE_PATH.as_path()).exists() {
        fs::File::create(FILE_PATH.as_path())?;
    }

    let stringified_quests = fs::read_to_string(FILE_PATH.as_path())?;
    let quest_list: QuestList = toml::from_str(&stringified_quests).unwrap_or_default();

    Ok(quest_list.quests)
}
