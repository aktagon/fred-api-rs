use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    // Key macro indicators Warren Buffett frequently references
    let indicators = vec![
        ("GDPC1", "Real Gross Domestic Product"),
        ("UNRATE", "Unemployment Rate"),
        ("FEDFUNDS", "Federal Funds Rate"),
        ("CPIAUCSL", "Consumer Price Index"),
        ("GDPPOT", "Real Potential GDP"),
        ("TB3MS", "3-Month Treasury Bill Rate"),
        ("DGS10", "10-Year Treasury Rate"),
        ("PAYEMS", "Total Nonfarm Payrolls"),
        ("INDPRO", "Industrial Production Index"),
        ("HOUST", "Housing Starts"),
    ];

    println!("=== Warren Buffett's Key Macro Economic Indicators ===\n");

    for (series_id, description) in indicators {
        println!("{}: {}", series_id, description);

        // Make direct HTTP request to get actual data since the API client only returns ()
        let url = format!(
            "https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type=json&limit=1&sort_order=desc",
            series_id, api_key
        );

        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(response) => match response.json::<Value>().await {
                Ok(json) => {
                    if let Some(observations) = json["observations"].as_array() {
                        if let Some(latest) = observations.first() {
                            let date = latest["date"].as_str().unwrap_or("N/A");
                            let value = latest["value"].as_str().unwrap_or("N/A");
                            println!("   Date: {} | Value: {}", date, value);
                        } else {
                            println!("   No observations available");
                        }
                    } else {
                        println!("   No observations in response");
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing JSON for {}: {:?}", series_id, e);
                }
            },
            Err(e) => {
                eprintln!("Error retrieving {}: {:?}", series_id, e);
            }
        }

        // Add a small delay to respect API rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("=== Summary ===");
    println!("These are the core macroeconomic indicators that Warren Buffett");
    println!("uses to gauge the health of the economy and inform his investment");
    println!("decisions. GDP growth, unemployment, and interest rates are");
    println!("particularly important for his long-term value investing approach.");

    Ok(())
}
