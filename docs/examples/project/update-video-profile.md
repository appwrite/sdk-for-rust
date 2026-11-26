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

    let result = project.update_video_profile(
        "<PROFILE_ID>",
        Some("<NAME>"), // optional
        Some(32000), // optional
        Some(32000), // optional
        Some(16), // optional
        Some(16) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
