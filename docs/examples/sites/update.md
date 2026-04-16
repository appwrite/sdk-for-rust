```rust
use appwrite::Client;
use appwrite::services::Sites;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let sites = Sites::new(&client);

    let result = sites.update(
        "<SITE_ID>",
        "<NAME>",
        appwrite::enums::Framework::Analog,
        Some(false), // optional
        Some(false), // optional
        Some(1), // optional
        Some("<INSTALL_COMMAND>"), // optional
        Some("<BUILD_COMMAND>"), // optional
        Some("<START_COMMAND>"), // optional
        Some("<OUTPUT_DIRECTORY>"), // optional
        Some(appwrite::enums::BuildRuntime::Node145), // optional
        Some(appwrite::enums::Adapter::Static), // optional
        Some("<FALLBACK_FILE>"), // optional
        Some("<INSTALLATION_ID>"), // optional
        Some("<PROVIDER_REPOSITORY_ID>"), // optional
        Some("<PROVIDER_BRANCH>"), // optional
        Some(false), // optional
        Some("<PROVIDER_ROOT_DIRECTORY>"), // optional
        Some(""), // optional
        Some(""), // optional
        Some(0) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
