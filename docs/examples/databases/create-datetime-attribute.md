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

    let result = databases.create_datetime_attribute(
        "<DATABASE_ID>",
        "<COLLECTION_ID>",
        "",
        false,
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
