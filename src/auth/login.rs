use crate::Login;
use crate::models;

use tonic::{Request, Response, Status};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use super::{LoginRequest, LoginResponse};

#[derive(Debug, Default)]
pub struct LoginService {}

#[tonic::async_trait]
impl Login for LoginService {
	async fn o_auth2(
		&self,
		request: Request<LoginRequest>
	) -> Result<Response<LoginResponse>,  Status> {
		println!("Got a request: {:?}", request);
		
		let req = request.into_inner();

		let reply = LoginResponse {
			token: get_jwt_for_user()
		};

		println!("Login email: {:?}\nLogin password: {:?}", req.email, req.password);
		Ok(Response::new(reply))
	}
}

fn get_jwt_for_user() -> String {
	let expiration_time = Utc::now()
			.checked_add_signed(Duration::seconds(60))
			.expect("invalid timestamp")
			.timestamp();

	let user_claims = models::user::Claims {
    sub: String::from("a"),
    role: String::from("a"),
    exp: expiration_time as usize,
	};

	let token = match encode(
		&Header::default(),
		&user_claims,
		&EncodingKey::from_secret(&String::from("secret").into_bytes()),
) {
		Ok(t) => t,
		Err(_) => panic!(),
};

	token
}