#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterSpecs {
    pub name: String,
    pub server: String,
    pub owned: bool,
}
