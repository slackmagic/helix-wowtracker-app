use crate::business::error::*;
use crate::business::traits::*;
use crate::core::character_data::CharacterData;
use crate::core::character_specs::CharacterSpecs;

use crate::storage::traits::StorageTrait;
use async_trait::async_trait;
use helix_tracker_lib::business::traits::TrackerDomainTrait;
use std::boxed::Box;

const WOW_CHAR_DATA_TYPE: &str = "wow_char_data";

pub struct WowTrackerDomain {
    storage: Box<dyn TrackerDomainTrait<CharacterSpecs, CharacterData>>,
}

impl WowTrackerDomain {
    pub fn new(storage: Box<dyn TrackerDomainTrait<CharacterSpecs, CharacterData>>) -> Self {
        WowTrackerDomain { storage: storage }
    }
}

#[async_trait]
impl WowTrackerDomainTrait for WowTrackerDomain {
    fn get_last_characters_data(
        &self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<Vec<CharacterData>> {
        let mut result = Vec::new();
        let char_data_list = self
            .storage
            .get_last_logs_by_type(&WOW_CHAR_DATA_TYPE.to_string(), owner_uuid)?;

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
        &mut self,
        owner_uuid: &uuid::Uuid,
    ) -> WowTrackerDomainResult<()> {
        Err(WowTrackerDomainError::NotImplemented)
    }
}
