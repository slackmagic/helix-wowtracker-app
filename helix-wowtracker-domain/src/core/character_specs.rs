use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterSpecs {
    pub id: Option<Uuid>,
    pub name: String,
    pub server: String,
    pub owned: bool,
}
