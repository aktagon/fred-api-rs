This edit is needed after generating the client:

```rust
350 -              ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
350 +              ContentType::Json => Ok(content),
```

