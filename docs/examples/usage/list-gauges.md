```rust
use appwrite::Client;
use appwrite::services::Usage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let usage = Usage::new(&client);

    let result = usage.list_gauges(
        vec![],
        Some(vec![]), // optional
        Some("1m"), // optional
        Some(vec![]), // optional
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some("time"), // optional
        Some("asc"), // optional
        Some(1), // optional
        Some(0) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
