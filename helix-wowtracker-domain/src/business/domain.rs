use crate::business::error::*;
use crate::business::traits::*;
use crate::core::character_data::CharacterData;
use crate::core::character_specs::CharacterSpecs;
use crate::storage::traits::*;

use async_trait::async_trait;
use helix_tracker_lib::business::traits::TrackerDomainTrait;
use std::boxed::Box;

const WOW_CHAR_DATA_TYPE: &str = "wow_char_data";

pub struct WowTrackerDomain {
    storage: Box<dyn TrackerDomainTrait<CharacterSpecs, CharacterData>>,
    blizzard_api: Box<dyn BlizzardAPIStorageTrait>,
}

impl WowTrackerDomain {
    pub fn new(
        storage: Box<dyn TrackerDomainTrait<CharacterSpecs, CharacterData>>,
        blizzard_api: Box<dyn BlizzardAPIStorageTrait>,
    ) -> Self {
        WowTrackerDomain {
            storage,
            blizzard_api,
        }
    }
}

#[async_trait]
impl WowTrackerDomainTrait for WowTrackerDomain {
    async fn get_last_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>> {
        let mut result = Vec::new();
        let char_data_list = self
            .storage
            .get_last_logs_by_type(&WOW_CHAR_DATA_TYPE.to_string(), owner_uuid)
            .await?;

        for log in char_data_list {
            match log.data {
                Some(mut data) => {
                    data.created_on = log.created_on;
                    result.push(data);
                }
                None => (),
            }
        }

        Ok(result)
    }

    async fn get_all_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>> {
        let mut result = Vec::new();
        let char_data_list = self
            .storage
            .get_logs_by_type(&WOW_CHAR_DATA_TYPE.to_string(), owner_uuid)
            .await?;

        for log in char_data_list {
            match log.data {
                Some(mut data) => {
                    data.created_on = log.created_on;
                    result.push(data);
                }
                None => (),
            }
        }

        Ok(result)
    }

    async fn update_all_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<()> {
        let char_spec = CharacterSpecs {
            name: "ölrinn".to_string(),
            server: "archimonde".to_string(),
            owned: true,
        };

        let result = self.blizzard_api.retrieve_character_data(&char_spec).await;
        match result {
            Ok(char_data) => Ok(()),
            Err(_) => Err(WowTrackerDomainError::StorageError),
        }
    }
}
