# FRED API Client for Rust

A Rust client library for the Federal Reserve Economic Data (FRED) API, providing access to economic data from the Federal Reserve Bank of St. Louis.

[![Crates.io](https://img.shields.io/crates/v/fred-api-client.svg)](https://crates.io/crates/fred-api-client)
[![Documentation](https://docs.rs/fred-api-client/badge.svg)](https://docs.rs/fred-api-client)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## About FRED

The Federal Reserve Economic Data (FRED) is a database of economic data maintained by the Federal Reserve Bank of St. Louis. It contains thousands of economic time series from scores of national, international, public, and private sources.

For more information about the FRED API, visit the [official documentation](https://fred.stlouisfed.org/docs/api/).

## Features

- **Comprehensive API Coverage**: Access to all FRED API endpoints including:
  - Categories and series data
  - Economic releases and sources
  - Tags and search functionality
  - Geographic data (GeoFRED)
- **Type-safe**: Fully typed Rust interface with proper error handling
- **Async Support**: Built on `reqwest` for asynchronous HTTP requests
- **Flexible Response Formats**: Support for XML and JSON response formats
- **Easy to Use**: Simple, intuitive API design

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fred-api-client = "1.0.0"
```

Or install using cargo:

```bash
cargo add fred-api-client
```

## Quick Start

First, you'll need to obtain a FRED API key from the [FRED website](https://fred.stlouisfed.org/docs/api/api_key.html).

```rust
use fred_api_client::{apis::default_api, apis::configuration::Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration {
        base_path: "https://api.stlouisfed.org".to_string(),
        ..Default::default()
    };

    // Get GDP series data
    let response = default_api::fred_series_get(
        &config,
        "your_api_key_here",
        "GDP",
        Some("json"),
        None,
        None,
    ).await?;

    println!("GDP Series Info: {}", response);
    Ok(())
}
```

## API Coverage

This client provides access to all FRED API endpoints:

### Categories
- Get categories and subcategories
- Search and filter categories
- Get series within categories

### Series
- Retrieve economic data series
- Get observations/data points
- Search series by text or tags
- Get series metadata and vintage dates

### Releases
- Access economic data releases
- Get release dates and sources
- Browse release-specific series

### Sources
- Get data sources information
- Browse source-specific releases

### Tags
- Search and filter by tags
- Get related tags
- Browse tagged series

### GeoFRED
- Geographic/regional economic data
- Shape files for mapping
- Regional data series

## API Key

You need a FRED API key to use this client. Get your free API key at:
https://fred.stlouisfed.org/docs/api/api_key.html

## Response Formats

The API supports multiple response formats:
- `xml` (default)
- `json`

Specify the format using the `file_type` parameter in API calls.

## Documentation

- [API Reference](docs/API.md) - Complete API endpoint documentation
- [FRED API Documentation](https://fred.stlouisfed.org/docs/api/) - Official FRED API docs
- Run `cargo doc --open` for local documentation

## Examples

Check out the `examples/` directory for more usage examples:

```bash
cargo run --example basic_usage
cargo run --example series_observations
cargo run --example category_search
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

- Commercial support available: christian@aktagon.com
- Issues and questions: [GitHub Issues](https://github.com/aktagon/edgar-rs/issues)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This is an unofficial client library. FRED® is a registered trademark of the Federal Reserve Bank of St. Louis.
