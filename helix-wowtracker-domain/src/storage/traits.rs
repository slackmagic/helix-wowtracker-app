use crate::core::app_user::*;
use crate::core::person::*;
use crate::storage::error::*;

pub trait StorageTrait: Send {
    fn login(&self, key: String) -> StorageResult<Option<AppUser>>;
    fn get_user(&self, uuid: &uuid::Uuid) -> StorageResult<Option<AppUser>>;
    fn get_all_users(&self) -> StorageResult<Vec<AppUser>>;
    fn create_user(&self, user: AppUser) -> StorageResult<AppUser>;
    fn update_user(&self, user: AppUser) -> StorageResult<AppUser>;
    fn delete_user(&self, user: AppUser) -> StorageResult<()>;

    fn create_person(&self, person: Person) -> StorageResult<Person>;
    fn update_person(&self, person: Person) -> StorageResult<Person>;
    fn delete_person(&self, person: Person) -> StorageResult<()>;
    fn get_person_by_uuid(&self, uuid: &uuid::Uuid) -> StorageResult<Option<Person>>;
    fn get_person_by_id(&self, id: i32) -> StorageResult<Option<Person>>;
    fn get_all_person(&self) -> StorageResult<Vec<Person>>;
}
