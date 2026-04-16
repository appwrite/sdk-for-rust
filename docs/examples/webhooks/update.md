```rust
use appwrite::Client;
use appwrite::services::Webhooks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let webhooks = Webhooks::new(&client);

    let result = webhooks.update(
        "<WEBHOOK_ID>",
        "<NAME>",
        "",
        vec![],
        Some(false), // optional
        Some(false), // optional
        Some("<AUTH_USERNAME>"), // optional
        Some("<AUTH_PASSWORD>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
