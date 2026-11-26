```rust
use appwrite::Client;
use appwrite::services::Organization;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let organization = Organization::new(&client);

    let result = organization.create_escalation(
        "<ESCALATION_ID>",
        "<NAME>",
        vec![serde_json::json!({"delay":0,"roles":["owner"]}), serde_json::json!({"delay":900,"roles":["developer"],"userIds":["<USER_ID>"]})],
        Some(1) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
