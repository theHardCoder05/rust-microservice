use std::sync::Mutex;
mod users;
mod sessions;

use users::UsersImpl;


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {

let users_service = Box::new(Mutex::new(UsersImpl::default()));

Ok(())


}