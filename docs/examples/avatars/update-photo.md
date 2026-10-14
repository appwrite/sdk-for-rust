```rust
use appwrite::Client;
use appwrite::services::Avatars;
use appwrite::InputFile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let avatars = Avatars::new(&client);

    let file = InputFile::from_path("path/to/file.png", None).await?;

    let result = avatars.update_photo(
        file
    ).await?;

    let _ = result;

    Ok(())
}
```
