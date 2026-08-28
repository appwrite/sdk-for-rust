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

    let result = postgresql.create(
        "<DATABASE_ID>",
        "<NAME>",
        Some("17"), // optional
        Some("<SPECIFICATION>"), // optional
        Some(0), // optional
        Some("async"), // optional
        Some(60), // optional
        Some(vec![]), // optional
        Some(5), // optional
        Some(false), // optional
        Some(1), // optional
        Some(false), // optional
        Some(50), // optional
        Some(0) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
