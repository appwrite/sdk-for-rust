```rust
use appwrite::Client;
use appwrite::services::Postgresql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let postgresql = Postgresql::new(&client);

    let result = postgresql.create_execution(
        "<DATABASE_ID>",
        "<SQL>",
        Some(serde_json::json!({})), // optional
        Some(1) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
