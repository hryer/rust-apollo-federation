pub struct Config {
  pub port: String,
  pub customer_service_address: String,
}

impl Config {
  pub fn new() -> Config {
    Self {
      port: std::env::var("PORT").unwrap_or_else(|_| String::from("8080")),
      customer_service_address: std::env::var("CUSTOMER_SERVICE_ADDRESS").unwrap_or_else(|e| {
        println!("CUSTOMER_SERVICE_ADDRESS env var is missing {}", e);
        std::process::exit(1)
      }),
    }
  }
}

lazy_static::lazy_static! {
pub static ref CONFIG: Config = Config::new();
}
