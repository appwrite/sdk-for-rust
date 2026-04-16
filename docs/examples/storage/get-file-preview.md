```rust
use appwrite::Client;
use appwrite::services::Storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let storage = Storage::new(&client);

    let result = storage.get_file_preview(
        "<BUCKET_ID>",
        "<FILE_ID>",
        Some(0), // optional
        Some(0), // optional
        Some(appwrite::enums::ImageGravity::Center), // optional
        Some(-1), // optional
        Some(0), // optional
        Some(""), // optional
        Some(0), // optional
        Some(0), // optional
        Some(-360), // optional
        Some(""), // optional
        Some(appwrite::enums::ImageFormat::Jpg), // optional
        Some("<TOKEN>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
