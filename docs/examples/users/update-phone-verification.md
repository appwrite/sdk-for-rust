```rust
use appwrite::Client;
use appwrite::services::Users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let users = Users::new(&client);

    let result = users.update_phone_verification(
        "<USER_ID>",
        false
    ).await?;

    let _ = result;

    Ok(())
}
```
