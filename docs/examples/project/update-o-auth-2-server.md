```rust
use appwrite::Client;
use appwrite::services::Project;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let project = Project::new(&client);

    let result = project.update_o_auth2_server(
        false,
        "https://example.com",
        Some(vec![]), // optional
        Some(vec![]), // optional
        Some(60), // optional
        Some(60), // optional
        Some(60), // optional
        Some(60), // optional
        Some(60), // optional
        Some(false), // optional
        Some("https://example.com"), // optional
        Some(6), // optional
        Some("numeric"), // optional
        Some(60), // optional
        Some(vec![]), // optional
        Some(vec![]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
