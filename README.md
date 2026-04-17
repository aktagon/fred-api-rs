# fred-api-rs

Rust client for the [FRED API](https://fred.stlouisfed.org/docs/api/) — economic time series from the Federal Reserve Bank of St. Louis. Async, typed, JSON.

Generated from the FRED OpenAPI spec. Covers categories, series, observations, releases, sources, tags, search, and GeoFRED.

## Install

Git dependency (not yet on crates.io):

```toml
[dependencies]
openapi = { git = "https://github.com/aktagon/fred-api-rs.git" }
tokio = { version = "1", features = ["full"] }
```

The generated crate is named `openapi`. See [examples/](examples/) for working usage.

## Quick Start

Get a [FRED API key](https://fred.stlouisfed.org/docs/api/api_key.html) and export it:

```bash
export FRED_API_KEY=your_key
```

```rust
use openapi::apis::{configuration::Configuration, default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FRED_API_KEY")?;
    let config = Configuration::new();

    // Real GDP, quarterly, last 10 observations
    default_api::fred_series_observations_get(
        &config, &api_key, "GDPC1",
        None, None, None, None, Some(10),
        None, None, None, None, None, None, None, None,
    ).await?;

    Ok(())
}
```

## Run the examples

```bash
export FRED_API_KEY=your_key
cargo run --example basic_usage
cargo run --example series_observations
cargo run --example buffett_macro_indicators
```

`buffett_macro_indicators` pulls Market-Cap-to-GDP inputs (Wilshire 5000 + GDP) — the macro ratio Buffett cited in 2001.

## API coverage

| Endpoint   | What you get                                                |
| ---------- | ----------------------------------------------------------- |
| Categories | Browse and search category trees, list series in a category |
| Series     | Observations, metadata, vintage dates, text and tag search  |
| Releases   | Release schedules, sources, related series                  |
| Sources    | Data source metadata and their releases                     |
| Tags       | Filter and browse by tag, find related tags                 |
| GeoFRED    | Regional economic data and shape files                      |

## Rate limits

FRED allows 120 requests per minute per API key. Exceeding this returns 429. The client does not throttle automatically — wrap calls in a limiter if you query in bulk.

## Documentation

- [API reference](docs/API.md)
- [FRED API docs](https://fred.stlouisfed.org/docs/api/)
- `cargo doc --open`

## License

MIT. Unofficial client. FRED is a registered trademark of the Federal Reserve Bank of St. Louis.

---

Built by [Aktagon](https://aktagon.com). Applied AI for regulated markets. Commercial support: [christian@aktagon.com](mailto:christian@aktagon.com).
