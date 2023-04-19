use std::sync::Mutex;

use crate::{users::Users};

use tonic::{Request, Response, Status};


use authentication::auth_server::Auth;
use authentication::{
    SignInRequest, SignInResponse, SignOutRequest, SignOutRequest, SignUpRequest, SignUpResponse, 
    StatusCode,
};

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub use authentication::auth_server::AuthServer;
pub use tonic::transport::Server;

pub struct AuthService {
    users_service: Box<Mutex<dyn Users + Send + Sync>>,
    sessions_service: Box<Mutex<dyn Users + Send + Sync>>,
}

impl AuthService {
    pub fn new(users_service: Box<Mutex<dyn Users + Send + Sync>>, sessions_service: Box<Mutex<dyn Users + Send + Sync>>) -> Self {
        Self {
            users_service,
            sessions_service,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>,Status>{
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let result = self
        .users_service
        .lock()
        .expect("lock should not be poisoned")
        .get_user_uuid(req.username, req.password);

        let user_uuid = match result {
            Some(uuid) => uuid,
            None => {
                let reply = SignInResponse {
                    status_code: StatusCode::Failure.into(),
                    user_uuid: "".to_owned(),
                    session_token: "".to_owned(),
                };

                return Ok(Response::new(reply));
            }
        };

    }
}

#[cfg(test)]
mod tests{
    use crate::{users::UsersImpl, sessions::SessionsImpl};

    use super::*;

    #[tokio::test]
    async fn sign_in_should_fail_if_user_not_found() {
        let user_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default(())));
        let auth_service = AuthService::new(user_service, session_service);

        let request = tonic::Request::new(SignInRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();
        assert_eq!(result.status_code, StatusCode::Failure.into());
        assert_eq!(result.user_uuid.is_empty(), true);
        assert_eq!(result.session_token.is_empty(), true);
    }
}