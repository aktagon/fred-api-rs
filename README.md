# fred-api-rs

Rust client for the [FRED API](https://fred.stlouisfed.org/docs/api/) — economic time series from the Federal Reserve. Async, typed, JSON only.

## About FRED

FRED is a database of economic time series maintained by the Federal Reserve Bank of St. Louis. Thousands of series from national, international, public, and private sources.

[Official API documentation](https://fred.stlouisfed.org/docs/api/)

## What it covers

Categories, series, observations, releases, sources, tags, search, and GeoFRED (regional data).

## Install

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

Get a [FRED API key](https://fred.stlouisfed.org/docs/api/api_key.html), then:

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

## API coverage

| Endpoint   | What you get                                                |
| ---------- | ----------------------------------------------------------- |
| Categories | Browse and search category trees, list series in a category |
| Series     | Observations, metadata, vintage dates, text/tag search      |
| Releases   | Release schedules, sources, related series                  |
| Sources    | Data source metadata and their releases                     |
| Tags       | Filter and browse by tag, find related tags                 |
| GeoFRED    | Regional economic data and shape files                      |

## Documentation

- [API Reference](docs/API.md)
- [FRED API docs](https://fred.stlouisfed.org/docs/api/)
- `cargo doc --open`

## Examples

Check out the `examples/` directory for more usage examples:

```bash
cargo run --example basic_usage
cargo run --example series_observations
cargo run --example category_search
```

## Support

- Commercial support: christian@aktagon.com
- Issues: [GitHub Issues](https://github.com/aktagon/fred-api-rs/issues)

## License

MIT. This is an unofficial client. FRED is a registered trademark of the Federal Reserve Bank of St. Louis.
