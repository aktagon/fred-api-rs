use openapi::apis::{configuration, default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    let config = configuration::Configuration::new();

    let category_id = 32991; // Real GDP category
    match default_api::fred_category_get(
        &config,
        &api_key,
        Some("json"),
        None,
        None,
        Some(category_id),
    )
    .await
    {
        Ok(response) => println!("Category info: {}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
