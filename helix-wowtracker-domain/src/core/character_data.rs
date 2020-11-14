use blizzard_api_rs::wow::character_equipment::CharacterEquipment;
use blizzard_api_rs::wow::character_media::CharacterMedia;
use blizzard_api_rs::wow::character_profile::CharacterProfile;
use blizzard_api_rs::wow::character_statistics::CharacterStatistics;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterData {
    pub created_on: Option<DateTime<Utc>>,
    pub profile: Option<CharacterProfile>,
    pub statistics: Option<CharacterStatistics>,
    pub equipment: Option<CharacterEquipment>,
    pub media: Option<CharacterMedia>,
}
