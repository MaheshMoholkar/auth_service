#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn get_user(){
        // get user
    }
    
    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }
}

mod auth_utils {

    pub fn login(creds: models::Credentials){
        crate::database::get_user();
    }
    
    fn logout(){
        // log out 
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials){
    if let database::Status::Connected = database::connect_to_database(){
        auth_utils::login(creds);
    }
}