```rust
use appwrite::Client;
use appwrite::services::Videos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let videos = Videos::new(&client);

    let result = videos.create_rendition(
        "<VIDEO_ID>",
        "<PROFILE_ID>",
        appwrite::enums::VideoOutput::Hls
    ).await?;

    let _ = result;

    Ok(())
}
```
