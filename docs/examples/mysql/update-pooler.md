```rust
use appwrite::Client;
use appwrite::services::Mysql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let mysql = Mysql::new(&client);

    let result = mysql.update_pooler(
        "<DATABASE_ID>",
        Some("transaction"), // optional
        Some(10), // optional
        Some(1), // optional
        Some(false), // optional
        Some("<POOLER_CPU_REQUEST>"), // optional
        Some("<POOLER_CPU_LIMIT>"), // optional
        Some("<POOLER_MEMORY_REQUEST>"), // optional
        Some("<POOLER_MEMORY_LIMIT>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
