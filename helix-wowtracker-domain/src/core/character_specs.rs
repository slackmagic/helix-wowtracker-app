#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecs {
    pub name: String,
    pub server: String,
    pub owned: bool,
}
