```rust
use appwrite::Client;
use appwrite::services::Analytics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let analytics = Analytics::new(&client);

    let result = analytics.create_event(
        "<PROPERTY_ID>",
        "<NAME>",
        "https://example.com",
        Some("<DOMAIN>"), // optional
        Some("<REFERRER>"), // optional
        Some(0), // optional
        Some("<SESSION_HASH>"), // optional
        Some(0), // optional
        Some(0), // optional
        Some(vec![]), // optional
        Some("<USER_ID>"), // optional
        Some("<IP>"), // optional
        Some("<USER_AGENT>"), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
