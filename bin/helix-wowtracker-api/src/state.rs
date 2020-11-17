use crate::configuration::Configuration;
use blizzard_api_storage::BlizzardApiStorage;
use helix_tracker_lib::business::domain::TrackerDomain;
use helix_tracker_lib::business::traits::TrackerDomainTrait;
use helix_tracker_lib::storage::pg_db_item_imp::PgDbItemTrackerStorage;
use helix_tracker_lib::storage::pg_db_log_imp::PgDbLogTrackerStorage;
use helix_tracker_lib::storage::traits::*;
use helix_wowtracker_domain::business::domain::WowTrackerDomain;
use helix_wowtracker_domain::business::traits::WowTrackerDomainTrait;
use helix_wowtracker_domain::core::character_data::CharacterData;
use helix_wowtracker_domain::core::character_specs::CharacterSpecs;
use helix_wowtracker_domain::storage::traits::BlizzardAPIStorageTrait;

use std::boxed::Box;

pub struct AppState {
    wow_tracker_domain: Box<dyn WowTrackerDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        let app_state = AppState {
            wow_tracker_domain: Box::new(WowTrackerDomain::new(
                AppState::get_tracker_storage(),
                AppState::get_blizzard_api_storage(),
            )),
        };

        app_state
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

    fn get_blizzard_api_storage() -> Box<dyn BlizzardAPIStorageTrait> {
        Box::new(BlizzardApiStorage::new(
            Configuration::get_blizzard_client(),
            Configuration::get_blizzard_secret(),
        ))
    }

    fn get_tracker_item_storage() -> Box<dyn ItemStorageTrait<CharacterSpecs>> {
        Box::new(PgDbItemTrackerStorage::<CharacterSpecs>::new(
            Configuration::get_database_name(),
            Configuration::get_database_host(),
            Configuration::get_database_port(),
            Configuration::get_database_user(),
            Configuration::get_database_password(),
        ))
    }

    fn get_tracker_log_storage() -> Box<dyn LogStorageTrait<CharacterData>> {
        Box::new(PgDbLogTrackerStorage::<CharacterData>::new(
            Configuration::get_database_name(),
            Configuration::get_database_host(),
            Configuration::get_database_port(),
            Configuration::get_database_user(),
            Configuration::get_database_password(),
        ))
    }
}
