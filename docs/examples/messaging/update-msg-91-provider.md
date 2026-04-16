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

    let result = messaging.update_msg91_provider(
        "<PROVIDER_ID>",
        Some("<NAME>"), // optional
        Some(false), // optional
        Some("<TEMPLATE_ID>"), // optional
        Some("<SENDER_ID>"), // optional
        Some("<AUTH_KEY>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
