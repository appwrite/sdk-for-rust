```rust
use appwrite::Client;
use appwrite::services::Avatars;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let avatars = Avatars::new(&client);

    let result = avatars.get_screenshot(
        "https://example.com",
        Some(serde_json::json!({})), // optional
        Some(1920), // optional
        Some(1080), // optional
        Some(2), // optional
        Some(appwrite::enums::Theme::Dark), // optional
        Some("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15"), // optional
        Some(true), // optional
        Some("en-US"), // optional
        Some(appwrite::enums::Timezone::AmericaNewYork), // optional
        Some(37.7749), // optional
        Some(-122.4194), // optional
        Some(100), // optional
        Some(true), // optional
        Some(vec![appwrite::enums::BrowserPermission::Geolocation, appwrite::enums::BrowserPermission::Notifications]), // optional
        Some(3), // optional
        Some(800), // optional
        Some(600), // optional
        Some(85), // optional
        Some(appwrite::enums::ImageFormat::Jpeg) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
