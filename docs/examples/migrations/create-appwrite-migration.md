```rust
use appwrite::Client;
use appwrite::services::Migrations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let migrations = Migrations::new(&client);

    let result = migrations.create_appwrite_migration(
        vec![appwrite::enums::AppwriteMigrationResource::User],
        "https://example.com",
        "<PROJECT_ID>",
        "<API_KEY>",
        Some(appwrite::enums::OnDuplicate::Fail) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
