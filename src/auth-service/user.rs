use pbkdf2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},Pbkdf2};
use rand_core::OsRng;
use uuid::Uuid;

use std::collections::HashMap;


/** Public interface */
pub trait Users{
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> ;
    fn get_user_uuid(&mut, username: String, password: String) -> Option<String>;
    fn delete_user(&mut, uuid_id: String);

}

#[derive(Clone)]
pub struct User{
    user_uuid: String,
    username: String,
    password: String,
}


#[derive(Default)]
pub struct UserImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

/** User struct to implement Users interface */
impl Users for UserImpl {

    /**
     * Create user function takes in UUID and Username as String
     */
    fn create_user(&mut self, uuid: String, username: String)-> Result<(), String>{
        if self.uuid_to_user.contains_key(&username) {
            return Err("Unable to create user. Username already exists".to_owned());
        }

        //Generate random salt from the crait
        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = Pbkdf2
                                .hashed_password(password.as_bytes(), &salt)
                                .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
                                .to_string();

        let user = User {
            uuid_to_user: Uuid::new_v4().to_string(),
            username: username.clone(),
            password: hashed_password,

        }

        self.username_to_user.insert(username, user);
        self.uuid_to_user.insert(user.user_uuid.clone(), user);

        return Ok(());
    }

    /**
     * Get User
     */
    fn get_user_uuid(&mut, username: String, password: String) -> Option<String> {
        let user = self.username_to_user.get(&username)?;

        let hashed_password = self.user.password.clone();
        let parsed_hash = PasswordHash::new(&hashed_password).ok()?;
        let result = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);

        if let user.username == username && result.is_ok() {
            return Some(user.user_uuid.clone());
        
        }

        None
    
    }


}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn should_create_user() {
        let mut user_service = UsersImpl::default();
        user_service
        .create_user("username".to_owned(), "password".to_owned())
        .expect("should create user");

     assert_eq!(user_service.uuid_to_user.len(),1);
     assert_eq!(user_service.username_to_user.len(),1);
    }
}