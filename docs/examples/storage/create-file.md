```rust
use appwrite::Client;
use appwrite::services::Storage;
use appwrite::InputFile;
use appwrite::permission::Permission;
use appwrite::role::Role;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let storage = Storage::new(&client);

    let file = InputFile::from_path("path/to/file.png", None).await?;

    let result = storage.create_file(
        "<BUCKET_ID>",
        "<FILE_ID>",
        file,
        Some(vec![Permission::read(Role::any()).to_string()]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
