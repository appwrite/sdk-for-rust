```rust
use appwrite::Client;
use appwrite::services::Functions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let functions = Functions::new(&client);

    let result = functions.create_execution(
        "<FUNCTION_ID>",
        Some("<BODY>"), // optional
        Some(false), // optional
        Some("<PATH>"), // optional
        Some(appwrite::enums::ExecutionMethod::GET), // optional
        Some(serde_json::json!({})), // optional
        Some("<SCHEDULED_AT>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
