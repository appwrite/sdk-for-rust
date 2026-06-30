```rust
use appwrite::Client;
use appwrite::services::Notifications;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let notifications = Notifications::new(&client);

    let result = notifications.update(
        "<NOTIFICATION_ID>",
        false
    ).await?;

    let _ = result;

    Ok(())
}
```
