```rust
use appwrite::Client;
use appwrite::services::Messaging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let messaging = Messaging::new(&client);

    let result = messaging.update_push(
        "<MESSAGE_ID>",
        Some(vec![]), // optional
        Some(vec![]), // optional
        Some(vec![]), // optional
        Some("<TITLE>"), // optional
        Some("<BODY>"), // optional
        Some(serde_json::json!({})), // optional
        Some("<ACTION>"), // optional
        Some("<ID1:ID2>"), // optional
        Some("<ICON>"), // optional
        Some("<SOUND>"), // optional
        Some("<COLOR>"), // optional
        Some("<TAG>"), // optional
        Some(0), // optional
        Some(false), // optional
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some(false), // optional
        Some(false), // optional
        Some(appwrite::enums::MessagePriority::Normal) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
