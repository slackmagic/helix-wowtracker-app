use crate::business::error::*;
use crate::core::character_data::CharacterData;
use async_trait::async_trait;

#[async_trait]
pub trait WowTrackerDomainTrait: Send + Sync {
    async fn get_previous_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>>;

    async fn get_last_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>>;

    async fn get_all_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>>;

    async fn update_all_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<()>;
}
