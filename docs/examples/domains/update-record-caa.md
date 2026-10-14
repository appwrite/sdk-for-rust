```rust
use appwrite::Client;
use appwrite::services::Domains;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let domains = Domains::new(&client);

    let result = domains.update_record_caa(
        "<DOMAIN_ID>",
        "<RECORD_ID>",
        "",
        "",
        1,
        Some("<COMMENT>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
