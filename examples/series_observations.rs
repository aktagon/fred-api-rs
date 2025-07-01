use openapi::apis::{configuration, default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    let config = configuration::Configuration::new();

    let series_id = "GDPC1"; // Real GDP series ID
    match default_api::fred_series_observations_get(
        &config,
        &api_key,
        series_id,
        None,
        None,
        None,
        None,
        Some(10),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    {
        Ok(_) => println!(
            "Successfully retrieved observations for series {}",
            series_id
        ),
        Err(e) => eprintln!("Error retrieving observations: {:?}", e),
    }

    Ok(())
}
