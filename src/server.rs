use tonic::{transport::Server, Request, Response, Status};

use auth::login_server::{Login, LoginServer};
use auth::{LoginRequest, LoginResponse};

pub mod auth {
  tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct LoginService {}

#[tonic::async_trait]
impl Login for LoginService {
	async fn test(
		&self,
		request: Request<LoginRequest>
	) -> Result<Response<LoginResponse, Status>> {
		println!("Got a request: {:?}", request);
		
		let req = request.into_inner();

		let reply = LoginResponse {
			token: "Token"
		};

		Ok(Response::new(reply))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let address = "[::1]:50051";
	let login_service: LoginService = LoginService::default();

	Server::builder()
		.add_service(LoginServer::new(login_service))
		.serve(addr)
		.await?;

	Ok(())
}