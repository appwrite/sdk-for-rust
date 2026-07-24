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

    let result = backups.create_restoration(
        "<ARCHIVE_ID>",
        vec![appwrite::enums::BackupServices::Databases],
        Some("<NEW_RESOURCE_ID>"), // optional
        Some("<NEW_RESOURCE_NAME>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
