```rust
use appwrite::Client;
use appwrite::services::Sites;
use appwrite::InputFile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let sites = Sites::new(&client);

    let code = InputFile::from_path("path/to/file.png", None).await?;

    let result = sites.create_deployment(
        "<SITE_ID>",
        code,
        Some("<INSTALL_COMMAND>"), // optional
        Some("<BUILD_COMMAND>"), // optional
        Some("<OUTPUT_DIRECTORY>"), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
