```rust
use appwrite::Client;
use appwrite::services::Mongo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let mongo = Mongo::new(&client);

    let result = mongo.update_backup_policy(
        "<DATABASE_ID>",
        "<POLICY_ID>",
        Some("<NAME>"), // optional
        Some(""), // optional
        Some(1), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
