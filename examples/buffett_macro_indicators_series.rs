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

    println!("=== Warren Buffett's Key Macro Economic Indicators - Full Series Analysis ===\n");

    for (series_id, description) in indicators {
        println!("{}: {}", series_id, description);
        println!("{}", "=".repeat(80));

        // Get full series data
        let full_url = format!(
            "https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type=json&sort_order=desc",
            series_id, api_key
        );

        let client = reqwest::Client::new();
        match client.get(&full_url).send().await {
            Ok(response) => match response.json::<Value>().await {
                Ok(json) => {
                    if let Some(observations) = json["observations"].as_array() {
                        let total_observations = observations.len();
                        println!("Total observations available: {}", total_observations);
                        
                        if total_observations > 0 {
                            // Show full series summary
                            let oldest = observations.last().unwrap();
                            let newest = observations.first().unwrap();
                            
                            let oldest_date = oldest["date"].as_str().unwrap_or("N/A");
                            let oldest_value = oldest["value"].as_str().unwrap_or("N/A");
                            let newest_date = newest["date"].as_str().unwrap_or("N/A");
                            let newest_value = newest["value"].as_str().unwrap_or("N/A");
                            
                            println!("Series range: {} to {}", oldest_date, newest_date);
                            println!("First value: {} | Latest value: {}", oldest_value, newest_value);
                            
                            // Show last 10 data points
                            println!("\nLast 10 observations:");
                            println!("{:<12} {:<15}", "Date", "Value");
                            println!("{}", "-".repeat(27));
                            
                            for observation in observations.iter().take(10) {
                                let date = observation["date"].as_str().unwrap_or("N/A");
                                let value = observation["value"].as_str().unwrap_or("N/A");
                                println!("{:<12} {:<15}", date, value);
                            }
                        } else {
                            println!(" No observations available");
                        }
                    } else {
                        println!(" No observations in response");
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

        println!("\n");
        // Add a small delay to respect API rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    println!("=== Summary ===");
    println!("This comprehensive view shows both the full historical range and recent");
    println!("data points for Warren Buffett's key economic indicators. The full series");
    println!("data helps understand long-term trends, while the last 10 observations");
    println!("provide insight into recent economic conditions and trajectory.");

    Ok(())
}
