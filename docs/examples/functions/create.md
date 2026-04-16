```rust
use appwrite::Client;
use appwrite::services::Functions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let functions = Functions::new(&client);

    let result = functions.create(
        "<FUNCTION_ID>",
        "<NAME>",
        appwrite::enums::Runtime::Node145,
        Some(vec!["any".into()]), // optional
        Some(vec![]), // optional
        Some(""), // optional
        Some(1), // optional
        Some(false), // optional
        Some(false), // optional
        Some("<ENTRYPOINT>"), // optional
        Some("<COMMANDS>"), // optional
        Some(vec![appwrite::enums::Scopes::SessionsWrite]), // optional
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
