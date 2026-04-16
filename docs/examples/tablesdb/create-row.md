```rust
use appwrite::Client;
use appwrite::services::TablesDB;
use appwrite::permission::Permission;
use appwrite::role::Role;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let tables_db = TablesDB::new(&client);

    let result = tables_db.create_row(
        "<DATABASE_ID>",
        "<TABLE_ID>",
        "<ROW_ID>",
        serde_json::json!({}),
        Some(vec![Permission::read(Role::any()).to_string()]), // optional
        Some("<TRANSACTION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
