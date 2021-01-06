use crate::core::character_data::CharacterData;
use crate::core::character_stats::CharacterStatistics;

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub data: Option<CharacterData>,
    pub statistics: Option<CharacterStatistics>,
}
