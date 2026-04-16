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
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let tables_db = TablesDB::new(&client);

    let result = tables_db.update_table(
        "<DATABASE_ID>",
        "<TABLE_ID>",
        Some("<NAME>"), // optional
        Some(vec![Permission::read(Role::any()).to_string()]), // optional
        Some(false), // optional
        Some(false), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
