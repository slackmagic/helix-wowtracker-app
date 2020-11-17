use async_trait::async_trait;
use blizzard_api_rs::battle_net::oauth_token::OAuthToken;
use blizzard_api_rs::BlizzardApiRS;
use helix_wowtracker_domain::core::character_data::CharacterData;
use helix_wowtracker_domain::core::character_specs::CharacterSpecs;
use helix_wowtracker_domain::storage::error::*;
use helix_wowtracker_domain::storage::traits::BlizzardAPIStorageTrait;
use std::sync::Arc;
use uuid::Uuid;

const API_REGION: &str = "eu";
const API_PROFILE: &str = "profile-eu";
const API_LANG: &str = "fr_FR";

pub struct BlizzardApiStorage {
    client: String,
    secret: String,
}

impl BlizzardApiStorage {
    pub fn new(client: String, secret: String) -> Self {
        BlizzardApiStorage { client, secret }
    }

    fn get_blizzard_api(&self) -> BlizzardApiRS {
        BlizzardApiRS::new(
            API_REGION.to_string(),
            API_PROFILE.to_string(),
            API_LANG.to_string(),
        )
    }

    async fn get_token(&self) -> StorageResult<OAuthToken> {
        match self
            .get_blizzard_api()
            .get_token(&self.client, &self.secret)
            .await
        {
            Ok(token) => Ok(token),
            Err(_) => Err(StorageError::ConnectionImpossible),
        }
    }

    async fn internal_retrieve_character_data(
        token: &OAuthToken,
        char_specs: &CharacterSpecs,
    ) -> StorageResult<CharacterData> {
        let api = BlizzardApiRS::new("eu".to_owned(), "profile-eu".to_owned(), "fr_FR".to_owned());
        let data_profile = api
            .get_wow_character_profile(&token, &char_specs.server, &char_specs.name)
            .await
            .unwrap();

        let data_statistics = api
            .get_wow_character_statistics(&token, &char_specs.server, &char_specs.name)
            .await
            .unwrap();

        let data_media = api
            .get_wow_character_media(&token, &char_specs.server, &char_specs.name)
            .await
            .unwrap();

        let data_equipment = api
            .get_wow_character_equipment(&token, &char_specs.server, &char_specs.name)
            .await
            .unwrap();

        Ok(CharacterData {
            created_on: None,
            profile: Some(data_profile),
            statistics: Some(data_statistics),
            equipment: Some(data_equipment),
            media: Some(data_media),
        })
    }

    async fn internal_retrieve_characters_data(
        token: &OAuthToken,
        chars_specs: Vec<(Uuid, CharacterSpecs)>,
    ) -> StorageResult<Vec<(Uuid, CharacterData)>> {
        let mut result: Vec<(Uuid, CharacterData)> = Vec::new();
        let arc_token: Arc<OAuthToken> = Arc::new(token.clone());
        let mut tasks: Vec<_> = Vec::new();

        for tuple in chars_specs {
            let cloned_token = arc_token.clone();
            let task = tokio::spawn(async move {
                let char_data =
                    BlizzardApiStorage::internal_retrieve_character_data(&cloned_token, &tuple.1)
                        .await
                        .unwrap();

                (tuple.0, char_data)
            });
            tasks.push(task);
        }

        for task in tasks {
            match task.await {
                Ok(retrieved_data) => result.push(retrieved_data),
                Err(_) => (),
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl BlizzardAPIStorageTrait for BlizzardApiStorage {
    async fn retrieve_character_data(
        &self,
        char_specs: &CharacterSpecs,
    ) -> StorageResult<CharacterData> {
        BlizzardApiStorage::internal_retrieve_character_data(&self.get_token().await?, char_specs)
            .await
    }

    async fn retrieve_characters_data(
        &self,
        chars_specs: Vec<(Uuid, CharacterSpecs)>,
    ) -> StorageResult<Vec<(Uuid, CharacterData)>> {
        BlizzardApiStorage::internal_retrieve_characters_data(&self.get_token().await?, chars_specs)
            .await
    }
}
