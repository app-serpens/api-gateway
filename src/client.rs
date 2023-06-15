use auth::login_client::LoginClient;
use auth::LoginRequest;

pub mod auth {
  tonic::include_proto!("auth");
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = LoginClient::connect(
    "http://[::1]:50051"
  ).await?;

  let request = tonic::Request::new(
    LoginRequest {
      email: "email",
      password: "password"
    }
  );

  let response = client.test(request).await?;
  println!("RESPONSE = {:?}", response);
  
  Ok(())
}