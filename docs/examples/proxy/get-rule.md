```rust
use appwrite::Client;
use appwrite::services::Proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let proxy = Proxy::new(&client);

    let result = proxy.get_rule(
        "<RULE_ID>"
    ).await?;

    let _ = result;

    Ok(())
}
```
