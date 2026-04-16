```rust
use appwrite::Client;
use appwrite::services::Databases;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let databases = Databases::new(&client);

    let result = databases.update_polygon_attribute(
        "<DATABASE_ID>",
        "<COLLECTION_ID>",
        "",
        false,
        Some(vec![serde_json::json!([[1,2],[3,4],[5,6],[1,2]])]), // optional
        Some("") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
