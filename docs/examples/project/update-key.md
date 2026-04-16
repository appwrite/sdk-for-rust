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

    let result = project.update_key(
        "<KEY_ID>",
        "<NAME>",
        vec![appwrite::enums::Scopes::SessionsWrite],
        Some("2020-10-15T06:38:00.000+00:00") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
