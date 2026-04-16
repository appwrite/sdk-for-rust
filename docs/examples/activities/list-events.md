```rust
use appwrite::Client;
use appwrite::services::Activities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let activities = Activities::new(&client);

    let result = activities.list_events(
        Some("") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
