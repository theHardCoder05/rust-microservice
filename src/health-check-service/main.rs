use std::env;
use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};

use uuid::Uuid;

use crate::authentication::StatusCode;

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());
    let mut client = AuthClient::connect(format!("http://{}:50051", hostname)).await?;

    /// While loop with sleep duration
    loop {   

        let username = Uuid::new_v4().to_string();
        let password = Uuid::new_v4().to_string();

        let request = tonic::Request::new(SignUpRequest{
            username: username.clone(),
            password: password.clone(),
        });

        let response =  client.sign_up(request).await?;

        println!("SIGN UP RESPONSE STATUS: {:?}", StatusCode::from_i32(response.into_inner().status_code()));

        

    }
}