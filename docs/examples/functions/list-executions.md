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

    let result = functions.list_executions(
        "<FUNCTION_ID>",
        Some(vec![]), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
