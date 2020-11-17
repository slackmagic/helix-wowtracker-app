use crate::core::character_data::CharacterData;
use crate::core::character_specs::CharacterSpecs;
use crate::storage::error::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BlizzardAPIStorageTrait: Send + Sync {
    async fn retrieve_character_data(
        &self,
        char_specs: &CharacterSpecs,
    ) -> StorageResult<CharacterData>;

    async fn retrieve_characters_data(
        &self,
        chars_specs: Vec<(Uuid, CharacterSpecs)>,
    ) -> StorageResult<Vec<(Uuid, CharacterData)>>;
}
