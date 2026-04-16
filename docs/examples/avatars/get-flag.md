```rust
use appwrite::Client;
use appwrite::services::Avatars;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let avatars = Avatars::new(&client);

    let result = avatars.get_flag(
        appwrite::enums::Flag::Afghanistan,
        Some(0), // optional
        Some(0), // optional
        Some(-1) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
