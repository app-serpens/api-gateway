use tonic::{Request, Response, Status};

use super::{LoginRequest, LoginResponse};

use crate::Login;

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
			token: String::from("Token")
		};

		println!("Login email: {:?}\nLogin password: {:?}", req.email, req.password);
		Ok(Response::new(reply))
	}
}