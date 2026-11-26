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

    let result = project.create_video_profile(
        "<PROFILE_ID>",
        "<NAME>",
        32000,
        32000,
        16,
        16,
        Some(appwrite::enums::VideoCodecId::H264) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
