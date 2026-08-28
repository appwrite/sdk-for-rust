```rust
use appwrite::Client;
use appwrite::services::VectorsDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let vectors_db = VectorsDB::new(&client);

    let result = vectors_db.get_document(
        "<DATABASE_ID>",
        "<COLLECTION_ID>",
        "<DOCUMENT_ID>",
        Some(vec![]), // optional
        Some("<TRANSACTION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
