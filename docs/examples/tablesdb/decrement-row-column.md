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

    let result = tables_db.decrement_row_column(
        "<DATABASE_ID>",
        "<TABLE_ID>",
        "<ROW_ID>",
        "",
        Some(0), // optional
        Some(0), // optional
        Some("<TRANSACTION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
