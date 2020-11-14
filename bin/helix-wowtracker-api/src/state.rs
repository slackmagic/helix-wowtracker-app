use crate::configuration::Configuration;
use helix_tracker_lib::business::domain::TrackerDomain;
use helix_tracker_lib::business::traits::TrackerDomainTrait;
use helix_tracker_lib::storage::pg_db_item_imp::PgDbItemTrackerStorage;
use helix_tracker_lib::storage::pg_db_log_imp::PgDbLogTrackerStorage;
use helix_tracker_lib::storage::traits::*;

use helix_wowtracker_domain::business::domain::WowTrackerDomain;
use helix_wowtracker_domain::business::traits::WowTrackerDomainTrait;
use helix_wowtracker_domain::core::character_data::CharacterData;
use helix_wowtracker_domain::core::character_specs::CharacterSpecs;
use std::boxed::Box;

pub struct AppState {
    wow_tracker_domain: Box<dyn WowTrackerDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            wow_tracker_domain: Box::new(WowTrackerDomain::new(AppState::get_tracker_storage())),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn WowTrackerDomainTrait + Send> {
        &self.wow_tracker_domain
    }

    fn get_tracker_storage() -> Box<dyn TrackerDomainTrait<CharacterSpecs, CharacterData>> {
        Box::new(TrackerDomain::new(
            AppState::get_tracker_item_storage(),
            AppState::get_tracker_log_storage(),
        ))
    }

    fn get_tracker_item_storage() -> Box<dyn ItemStorageTrait<CharacterSpecs>> {
        Box::new(PgDbItemTrackerStorage::<CharacterSpecs>::new(
            Configuration::get_database_connection_string(),
        ))
    }

    fn get_tracker_log_storage() -> Box<dyn LogStorageTrait<CharacterData>> {
        Box::new(PgDbLogTrackerStorage::<CharacterData>::new(
            Configuration::get_database_connection_string(),
        ))
    }
}
