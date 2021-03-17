use quest::{DynResult, Quest, QuestList};
use std::io;

/// Saved quests file path
static FILE_PATH: &str = "./quests.toml";

/// Save all quests to a file
pub fn save_quests(quests: &[Quest]) -> DynResult {
    let quests = &QuestList::new(quests);
    let stringified_quests = toml::to_string(quests)?;
    std::fs::write(FILE_PATH, stringified_quests)?;

    Ok(())
}

/// Load all saved quests from file
pub fn load_quests() -> Result<Vec<Quest>, io::Error> {
    let stringified_quests = std::fs::read_to_string(FILE_PATH)?;
    let quest_list: QuestList = toml::from_str(&stringified_quests).unwrap_or_default();

    Ok(quest_list.quests)
}
