use tonic::{transport::Server};

use auth::login_server::{Login, LoginServer};

use auth::login::LoginService as LoginService;

pub mod auth {
  tonic::include_proto!("auth");
	pub mod login;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let address = "[::1]:50051".parse()?;
	let login_service: LoginService = LoginService::default();

	Server::builder()
		.add_service(LoginServer::new(login_service))
		.serve(address)
		.await?;

	Ok(())
}