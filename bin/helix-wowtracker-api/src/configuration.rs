use std::env;

pub struct Configuration {}

impl Configuration {
    pub fn get_database_connection_string() -> String {
        env::var("HELIX_DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING not found.")
    }

    pub fn get_blizzard_client() -> String {
        env::var("BLIZZARD_CLIENT").expect("BLIZZARD_CLIENT not found.")
    }
    pub fn get_blizzard_secret() -> String {
        env::var("BLIZZARD_SECRET").expect("BLIZZARD_SECRET not found.")
    }

    pub fn get_database_name() -> String {
        env::var("HELIX_DB_NAME").expect("HELIX_DB_NAME not found.")
    }

    pub fn get_database_host() -> String {
        env::var("HELIX_DB_HOST").expect("HELIX_DB_HOST not found.")
    }

    pub fn get_database_port() -> u16 {
        env::var("HELIX_DB_PORT")
            .expect("HELIX_DB_PORT not found.")
            .parse()
            .unwrap()
    }

    pub fn get_database_user() -> String {
        env::var("HELIX_DB_USER").expect("BLIZZARD_SECRET not found.")
    }

    pub fn get_database_password() -> String {
        env::var("HELIX_DB_PASSWORD").expect("BLIZZARD_SECRET not found.")
    }
}
