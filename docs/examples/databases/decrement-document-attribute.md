```rust
use appwrite::Client;
use appwrite::services::Databases;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let databases = Databases::new(&client);

    let result = databases.decrement_document_attribute(
        "<DATABASE_ID>",
        "<COLLECTION_ID>",
        "<DOCUMENT_ID>",
        "",
        Some(0), // optional
        Some(0), // optional
        Some("<TRANSACTION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
