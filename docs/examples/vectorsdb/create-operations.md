```rust
use appwrite::Client;
use appwrite::services::VectorsDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let vectors_db = VectorsDB::new(&client);

    let result = vectors_db.create_operations(
        "<TRANSACTION_ID>",
        Some(vec![serde_json::json!({"action":"create","databaseId":"<DATABASE_ID>","collectionId":"<COLLECTION_ID>","documentId":"<DOCUMENT_ID>","data":{"name":"Walter O'Brien"}})]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
