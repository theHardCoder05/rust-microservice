
mod users;
use users::UserImpl;

fn main(){

let users_service = Box::new(UserImpl::default());
}