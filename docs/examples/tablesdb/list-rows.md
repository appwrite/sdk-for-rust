```rust
use appwrite::Client;
use appwrite::services::TablesDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let tables_db = TablesDB::new(&client);

    let result = tables_db.list_rows(
        "<DATABASE_ID>",
        "<TABLE_ID>",
        Some(vec![]), // optional
        Some("<TRANSACTION_ID>"), // optional
        Some(false), // optional
        Some(0) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
