```rust
use appwrite::Client;
use appwrite::services::Organization;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let organization = Organization::new(&client);

    let result = organization.create_membership(
        vec![],
        Some("email@example.com"), // optional
        Some("<USER_ID>"), // optional
        Some("+12065550100"), // optional
        Some("https://example.com"), // optional
        Some("<NAME>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
