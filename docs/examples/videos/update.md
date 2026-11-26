```rust
use appwrite::Client;
use appwrite::services::Videos;
use appwrite::permission::Permission;
use appwrite::role::Role;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let videos = Videos::new(&client);

    let result = videos.update(
        "<VIDEO_ID>",
        Some("<NAME>"), // optional
        Some(vec![Permission::read(Role::any()).to_string()]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
