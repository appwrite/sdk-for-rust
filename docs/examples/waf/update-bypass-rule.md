```rust
use appwrite::Client;
use appwrite::services::Waf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let waf = Waf::new(&client);

    let result = waf.update_bypass_rule(
        "<RULE_ID>",
        Some("api"), // optional
        Some("<RESOURCE_ID>"), // optional
        Some("<NAME>"), // optional
        Some("<DESCRIPTION>"), // optional
        Some(-100000), // optional
        Some(false), // optional
        Some(vec![]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
