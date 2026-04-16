```rust
use appwrite::Client;
use appwrite::services::Functions;
use appwrite::InputFile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let functions = Functions::new(&client);

    let code = InputFile::from_path("path/to/file.png", None).await?;

    let result = functions.create_deployment(
        "<FUNCTION_ID>",
        code,
        false,
        Some("<ENTRYPOINT>"), // optional
        Some("<COMMANDS>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
