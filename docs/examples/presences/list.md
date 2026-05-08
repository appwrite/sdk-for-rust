```rust
use appwrite::Client;
use appwrite::services::Presences;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let presences = Presences::new(&client);

    let result = presences.list(
        Some(vec![]), // optional
        Some(false), // optional
        Some(0) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
