```rust
use appwrite::Client;
use appwrite::services::Messaging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_jwt("<YOUR_JWT>"); // Your secret JSON Web Token

    let messaging = Messaging::new(&client);

    let result = messaging.create_subscriber(
        "<TOPIC_ID>",
        "<SUBSCRIBER_ID>",
        "<TARGET_ID>"
    ).await?;

    let _ = result;

    Ok(())
}
```
