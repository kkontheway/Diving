use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sts::Client;
// use aws_types::region::Region;
// use clap::Parser;
use dotenv::dotenv;
use std::fs::File;
use std::io::Write;
use tokio;

#[tokio::main]
async fn main() {
    if let Err(_) = dotenv() {
        println!(".env file not found. Creating a new one...");
        let mut file = File::create(".env").expect("Failed to create .env file");
        file.write_all(b"AWS_ACCESS_KEY_ID=YOUR_ACCESS_KEY_ID\n")
            .expect("Failed to write to .env file");
        file.write_all(b"AWS_SECRET_ACCESS_KEY=YOUR_SECRET_ACCESS_KEY\n")
            .expect("Failed to write to .env file");
        file.write_all(b"AWS_REGION=YOUR_REGION\n")
            .expect("Failed to write to .env file");
        println!(".env file created successfully.");
    }
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    match client.get_caller_identity().send().await {
        Ok(resp) => {
            println!("User: {}", resp.arn().unwrap_or_default());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
