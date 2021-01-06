#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterStatistics {
    pub level_current_progression: f32,
    pub level_progression_change: u16,
    pub ilevel_progression_change: u16,
    pub ilevel_progression_rate: u16,
}
