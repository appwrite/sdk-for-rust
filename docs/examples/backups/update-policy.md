```rust
use appwrite::Client;
use appwrite::services::Backups;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let backups = Backups::new(&client);

    let result = backups.update_policy(
        "<POLICY_ID>",
        Some("<NAME>"), // optional
        Some(1), // optional
        Some(""), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
