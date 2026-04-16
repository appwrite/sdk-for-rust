```rust
use appwrite::Client;
use appwrite::services::Graphql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let graphql = Graphql::new(&client);

    graphql.mutation(
        serde_json::json!({})
    ).await?;

    Ok(())
}
```
